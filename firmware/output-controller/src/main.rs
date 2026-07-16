#![no_std]
#![no_main]

use board_common::{clock_config, usart_config, IWDG_TIMEOUT_US, UART_RX_BUF_LEN};
use embassy_executor::Spawner;
use embassy_futures::select::{select4, select_array, Either4};
use embassy_stm32::exti::{ExtiInput, InterruptHandler as ExtiInterruptHandler};
use embassy_stm32::gpio::{Level, Output, Pull, Speed};
use embassy_stm32::mode::Async;
use embassy_stm32::usart::{Uart, UartRx, UartTx};
use embassy_stm32::wdg::IndependentWatchdog;
use embassy_stm32::{bind_interrupts, dma, interrupt, peripherals, usart};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::watch::Watch;
use embassy_time::{Instant, Timer};

use hotline_protocol::{
    resolve_outputs, DecoderEvent, FrameDecoder, HeartbeatFrame, COMM_WATCHDOG_TIMEOUT_MS,
    HEARTBEAT_INTERVAL_MS,
};

use defmt_rtt as _;
use panic_probe as _;

// Pin map: hardware/v1.0.0/PIN_MAP.md
//   OVR CH0..CH7 -> PA0..PA7          (Schmitt HIGH = force ON)
//   GATE CH0..7  -> PB0, PB1, PB2, PB10..PB14 (HIGH = MOSFET on)
//   CH LEDs      -> PB3..PB9, PC6     (active-low; refs CH0..CH7)
//   LINK LED     -> PC7 (active-low; ref LINK)
//   USART1 TX/RX -> PA9 / PA10        (U2A.DI / U2B.RO) — same as input board
//   USB          -> PA11/PA12 ; SWD on J8 ; BOOT0=SW2, NRST=SW1

bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
    DMA1_CHANNEL2_3 => dma::InterruptHandler<peripherals::DMA1_CH2>, dma::InterruptHandler<peripherals::DMA1_CH3>;
    EXTI0_1 => ExtiInterruptHandler<interrupt::typelevel::EXTI0_1>;
    EXTI2_3 => ExtiInterruptHandler<interrupt::typelevel::EXTI2_3>;
    EXTI4_15 => ExtiInterruptHandler<interrupt::typelevel::EXTI4_15>;
});

/// Upper bound on output-driver loop period: guarantees the watchdog is petted and
/// outputs are refreshed even with no serial/override/failsafe activity.
const OUTPUT_REEVAL_MS: u64 = 200;

/// Last valid channel state received over serial.
static SERIAL_STATE: Watch<CriticalSectionRawMutex, u8, 3> = Watch::new();

/// Timestamp of last valid state frame (for comm watchdog).
static LAST_FRAME_INSTANT: Watch<CriticalSectionRawMutex, Instant, 2> = Watch::new();

/// Whether the communication watchdog has fired (failsafe mode).
static FAILSAFE_ACTIVE: Watch<CriticalSectionRawMutex, bool, 3> = Watch::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(clock_config());

    let mut iwdg = IndependentWatchdog::new(p.IWDG, IWDG_TIMEOUT_US);
    iwdg.unleash();

    let uart1 = Uart::new(
        p.USART1,
        p.PA10,
        p.PA9,
        p.DMA1_CH2,
        p.DMA1_CH3,
        Irqs,
        usart_config(),
    )
    .unwrap();
    let (tx, rx) = uart1.split();

    let mut overrides = [
        ExtiInput::new(p.PA0, p.EXTI0, Pull::None, Irqs), // OVR0 / J3a.1
        ExtiInput::new(p.PA1, p.EXTI1, Pull::None, Irqs), // OVR1 / J3a.2
        ExtiInput::new(p.PA2, p.EXTI2, Pull::None, Irqs), // OVR2 / J3a.3
        ExtiInput::new(p.PA3, p.EXTI3, Pull::None, Irqs), // OVR3 / J3a.4
        ExtiInput::new(p.PA4, p.EXTI4, Pull::None, Irqs), // OVR4 / J3b.1
        ExtiInput::new(p.PA5, p.EXTI5, Pull::None, Irqs), // OVR5 / J3b.2
        ExtiInput::new(p.PA6, p.EXTI6, Pull::None, Irqs), // OVR6 / J3b.3
        ExtiInput::new(p.PA7, p.EXTI7, Pull::None, Irqs), // OVR7 / J3b.4
    ];

    let mut gates: [Output; 8] = [
        Output::new(p.PB0, Level::Low, Speed::Low),  // Q1 / J5a.1
        Output::new(p.PB1, Level::Low, Speed::Low),  // Q2 / J5a.2
        Output::new(p.PB2, Level::Low, Speed::Low),  // Q3 / J5a.3
        Output::new(p.PB10, Level::Low, Speed::Low), // Q4 / J5a.4
        Output::new(p.PB11, Level::Low, Speed::Low), // Q5 / J5b.1
        Output::new(p.PB12, Level::Low, Speed::Low), // Q6 / J5b.2
        Output::new(p.PB13, Level::Low, Speed::Low), // Q7 / J5b.3
        Output::new(p.PB14, Level::Low, Speed::Low), // Q8 / J5b.4
    ];

    let mut ch_leds: [Output; 8] = [
        Output::new(p.PB3, Level::High, Speed::Low), // CH0
        Output::new(p.PB4, Level::High, Speed::Low), // CH1
        Output::new(p.PB5, Level::High, Speed::Low), // CH2
        Output::new(p.PB6, Level::High, Speed::Low), // CH3
        Output::new(p.PB7, Level::High, Speed::Low), // CH4
        Output::new(p.PB8, Level::High, Speed::Low), // CH5
        Output::new(p.PB9, Level::High, Speed::Low), // CH6
        Output::new(p.PC6, Level::High, Speed::Low), // CH7
    ];

    let led_link = Output::new(p.PC7, Level::High, Speed::Low);

    SERIAL_STATE.sender().send(0u8);
    LAST_FRAME_INSTANT.sender().send(Instant::now());
    FAILSAFE_ACTIVE.sender().send(false);

    defmt::info!("output-controller: init complete");

    spawner.spawn(serial_receive(rx).unwrap());
    spawner.spawn(heartbeat_transmit(tx).unwrap());
    spawner.spawn(comm_watchdog().unwrap());
    spawner.spawn(link_led_driver(led_link).unwrap());

    let mut serial_rx = SERIAL_STATE.receiver().unwrap();
    let mut failsafe_rx = FAILSAFE_ACTIVE.receiver().unwrap();
    let mut serial_channels: u8 = serial_rx.try_get().unwrap_or(0);
    let mut failsafe: bool = failsafe_rx.try_get().unwrap_or(false);

    loop {
        let mut override_bits: u8 = 0;
        for (i, input) in overrides.iter().enumerate() {
            if input.is_high() {
                override_bits |= 1 << i;
            }
        }

        let outputs = resolve_outputs(failsafe, override_bits, serial_channels);
        for i in 0..8usize {
            if outputs & (1 << i) != 0 {
                gates[i].set_high();
                ch_leds[i].set_low();
            } else {
                gates[i].set_low();
                ch_leds[i].set_high();
            }
        }

        iwdg.pet();

        let override_edges = overrides.each_mut().map(|o| o.wait_for_any_edge());
        match select4(
            serial_rx.changed(),
            failsafe_rx.changed(),
            select_array(override_edges),
            Timer::after_millis(OUTPUT_REEVAL_MS),
        )
        .await
        {
            Either4::First(s) => serial_channels = s,
            Either4::Second(f) => failsafe = f,
            Either4::Third(_) => {}  // override edge; re-read at loop top
            Either4::Fourth(_) => {} // periodic safety re-eval
        }
    }
}

#[embassy_executor::task]
async fn serial_receive(mut rx: UartRx<'static, Async>) {
    defmt::info!("serial_receive: started");
    let mut decoder = FrameDecoder::new();
    let mut buf = [0u8; UART_RX_BUF_LEN];
    let state_tx = SERIAL_STATE.sender();
    let frame_ts_tx = LAST_FRAME_INSTANT.sender();

    loop {
        match rx.read_until_idle(&mut buf).await {
            Ok(n) => {
                for &byte in &buf[..n] {
                    match decoder.feed(byte) {
                        DecoderEvent::State(frame) => {
                            state_tx.send(frame.channels);
                            frame_ts_tx.send(Instant::now());
                        }
                        DecoderEvent::CrcError => {
                            defmt::warn!("rx: CRC error");
                        }
                        _ => {}
                    }
                }
            }
            Err(e) => {
                defmt::warn!("rx err: {:?}", e);
            }
        }
    }
}

#[embassy_executor::task]
async fn heartbeat_transmit(mut tx: UartTx<'static, Async>) {
    defmt::info!("heartbeat_transmit: started (10 Hz)");
    let mut failsafe_rx = FAILSAFE_ACTIVE.receiver().unwrap();

    loop {
        Timer::after_millis(HEARTBEAT_INTERVAL_MS).await;

        let failsafe = failsafe_rx.try_get().unwrap_or(false);
        let hb = HeartbeatFrame::new(!failsafe, failsafe);
        let wire = hb.encode();

        if let Err(e) = tx.write(&wire).await {
            defmt::warn!("hb TX err: {:?}", e);
        }
    }
}

#[embassy_executor::task]
async fn comm_watchdog() {
    defmt::info!(
        "comm_watchdog: started ({}ms timeout)",
        COMM_WATCHDOG_TIMEOUT_MS
    );
    let failsafe_tx = FAILSAFE_ACTIVE.sender();
    let mut ts_rx = LAST_FRAME_INSTANT.receiver().unwrap();
    let mut was_failsafe = false;

    loop {
        Timer::after_millis(COMM_WATCHDOG_TIMEOUT_MS / 2).await;
        let last_ts = ts_rx.try_get().unwrap_or(Instant::from_millis(0));
        let elapsed = last_ts.elapsed().as_millis();
        let failsafe = elapsed > COMM_WATCHDOG_TIMEOUT_MS;

        if failsafe != was_failsafe {
            was_failsafe = failsafe;
            failsafe_tx.send(failsafe);
            if failsafe {
                defmt::warn!("FAILSAFE: no frame for {}ms", elapsed);
            } else {
                defmt::info!("FAILSAFE cleared");
            }
        }
    }
}

#[embassy_executor::task]
async fn link_led_driver(mut led: Output<'static>) {
    let mut failsafe_rx = FAILSAFE_ACTIVE.receiver().unwrap();
    let mut in_failsafe = true; // start pessimistic until first frame

    loop {
        if let Some(f) = failsafe_rx.try_get() {
            in_failsafe = f;
        }

        if in_failsafe {
            led.set_low();
            Timer::after_millis(50).await;
            led.set_high();
            Timer::after_millis(50).await;
        } else {
            led.set_low();
            Timer::after_millis(200).await;
        }
    }
}
