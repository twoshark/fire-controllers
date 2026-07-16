#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_futures::select::{select4, select_array, Either4};
use embassy_stm32::exti::{ExtiInput, InterruptHandler as ExtiInterruptHandler};
use embassy_stm32::gpio::{Level, Output, Pull, Speed};
use embassy_stm32::mode::Async;
use embassy_stm32::usart::{Config as UartConfig, Uart, UartRx, UartTx};
use embassy_stm32::wdg::IndependentWatchdog;
use embassy_stm32::{bind_interrupts, dma, interrupt, peripherals, usart};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::watch::Watch;
use embassy_time::{Instant, Timer};

use hotline_protocol::{
    DecoderEvent, FrameDecoder, HeartbeatFrame, COMM_WATCHDOG_TIMEOUT_MS, HEARTBEAT_INTERVAL_MS,
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
    // EXTI lines 0..7 (one per override channel PA0..PA7) grouped into the G0 vectors.
    EXTI0_1 => ExtiInterruptHandler<interrupt::typelevel::EXTI0_1>;
    EXTI2_3 => ExtiInterruptHandler<interrupt::typelevel::EXTI2_3>;
    EXTI4_15 => ExtiInterruptHandler<interrupt::typelevel::EXTI4_15>;
});

/// Upper bound on output-driver loop period: guarantees the watchdog is petted and
/// outputs are refreshed even with no serial/override/failsafe activity.
const OUTPUT_REEVAL_MS: u64 = 200;

// ---------------------------------------------------------------------------
// Shared state (statics)
// ---------------------------------------------------------------------------

/// Last valid channel state received over serial.
static SERIAL_STATE: Watch<CriticalSectionRawMutex, u8, 3> = Watch::new();

/// Timestamp of last valid state frame (for comm watchdog).
static LAST_FRAME_INSTANT: Watch<CriticalSectionRawMutex, Instant, 2> = Watch::new();

/// Whether the communication watchdog has fired (failsafe mode).
static FAILSAFE_ACTIVE: Watch<CriticalSectionRawMutex, bool, 3> = Watch::new();

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let mut config = embassy_stm32::Config::default();
    {
        use embassy_stm32::rcc::*;
        config.rcc.hsi = Some(Hsi {
            sys_div: HsiSysDiv::DIV1,
        });
        config.rcc.pll = Some(Pll {
            source: PllSource::HSI,
            prediv: PllPreDiv::DIV1,
            mul: PllMul::MUL8,
            divp: None,
            divq: None,
            divr: Some(PllRDiv::DIV2),
        });
        config.rcc.sys = Sysclk::PLL1_R;
    }
    let p = embassy_stm32::init(config);

    // -- IWDG: 1 s hardware watchdog --
    let mut iwdg = IndependentWatchdog::new(p.IWDG, 1_000_000);
    iwdg.unleash();

    // -- USART1: RS-485 at 115200 baud (PA9=TX, PA10=RX) with DMA --
    let mut uart_config = UartConfig::default();
    uart_config.baudrate = 115200;
    let uart1 = Uart::new(
        p.USART1,
        p.PA10,
        p.PA9,
        p.DMA1_CH2,
        p.DMA1_CH3,
        Irqs,
        uart_config,
    )
    .unwrap();
    let (tx, rx) = uart1.split();

    // -- Overrides: J3a CH0..3 + J3b CH4..7 -> PA0..PA7 (HIGH = force ON) --
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

    // -- MOSFET gates (default LOW = off; HW 10k PD R9..R16, no series Rg) --
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

    // -- Channel LEDs CH0..CH7 (active LOW) --
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

    // -- LINK LED (active LOW): PC7 --
    let led_link = Output::new(p.PC7, Level::High, Speed::Low);

    // Initialise shared state
    SERIAL_STATE.sender().send(0u8);
    LAST_FRAME_INSTANT.sender().send(Instant::now());
    FAILSAFE_ACTIVE.sender().send(false);

    defmt::info!("output-controller: init complete");

    // Spawn tasks
    spawner.spawn(serial_receive(rx).unwrap());
    spawner.spawn(heartbeat_transmit(tx).unwrap());
    spawner.spawn(comm_watchdog().unwrap());
    spawner.spawn(link_led_driver(led_link).unwrap());

    let mut serial_rx = SERIAL_STATE.receiver().unwrap();
    let mut failsafe_rx = FAILSAFE_ACTIVE.receiver().unwrap();
    let mut serial_channels: u8 = serial_rx.try_get().unwrap_or(0);
    let mut failsafe: bool = failsafe_rx.try_get().unwrap_or(false);

    loop {
        // Read 8 override digital inputs (closed switch = HIGH = force ON).
        let mut override_on = [false; 8];
        for (i, input) in overrides.iter().enumerate() {
            override_on[i] = input.is_high();
        }

        // Merge: failsafe forces all OFF; otherwise override forces ON, else the
        // channel follows the latest serial state. One pass drives every gate + LED.
        for i in 0..8usize {
            let on = if failsafe {
                false
            } else if override_on[i] {
                true
            } else {
                serial_channels & (1 << i) != 0
            };

            if on {
                gates[i].set_high();
                ch_leds[i].set_low(); // LED ON
            } else {
                gates[i].set_low();
                ch_leds[i].set_high(); // LED OFF
            }
        }

        iwdg.pet();

        // Re-evaluate immediately on a new serial frame, a failsafe transition, or
        // any override edge. The timeout bounds the loop so the watchdog is petted
        // and outputs refreshed even with no activity.
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

// ---------------------------------------------------------------------------
// Serial receive task (continuous frame RX)
// ---------------------------------------------------------------------------

#[embassy_executor::task]
async fn serial_receive(mut rx: UartRx<'static, Async>) {
    defmt::info!("serial_receive: started");
    let mut decoder = FrameDecoder::new();
    let mut buf = [0u8; 1];
    let state_tx = SERIAL_STATE.sender();
    let frame_ts_tx = LAST_FRAME_INSTANT.sender();

    loop {
        match rx.read(&mut buf).await {
            Ok(()) => match decoder.feed(buf[0]) {
                DecoderEvent::State(frame) => {
                    state_tx.send(frame.channels);
                    frame_ts_tx.send(Instant::now());
                }
                DecoderEvent::CrcError => {
                    defmt::warn!("rx: CRC error");
                }
                _ => {}
            },
            Err(e) => {
                defmt::warn!("rx err: {:?}", e);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Heartbeat transmit task (10 Hz)
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Communication watchdog task
// ---------------------------------------------------------------------------

#[embassy_executor::task]
async fn comm_watchdog() {
    defmt::info!(
        "comm_watchdog: started ({}ms timeout)",
        COMM_WATCHDOG_TIMEOUT_MS
    );
    let failsafe_tx = FAILSAFE_ACTIVE.sender();
    let mut ts_rx = LAST_FRAME_INSTANT.receiver().unwrap();

    loop {
        Timer::after_millis(COMM_WATCHDOG_TIMEOUT_MS / 2).await;
        let last_ts = ts_rx.try_get().unwrap_or(Instant::from_millis(0));
        let elapsed = last_ts.elapsed().as_millis();

        if elapsed > COMM_WATCHDOG_TIMEOUT_MS {
            failsafe_tx.send(true);
            defmt::warn!("FAILSAFE: no frame for {}ms", elapsed);
        } else {
            failsafe_tx.send(false);
        }
    }
}

// ---------------------------------------------------------------------------
// Link LED driver task
// ---------------------------------------------------------------------------

#[embassy_executor::task]
async fn link_led_driver(mut led: Output<'static>) {
    let mut failsafe_rx = FAILSAFE_ACTIVE.receiver().unwrap();
    let mut in_failsafe = true; // start pessimistic until first frame

    loop {
        if let Some(f) = failsafe_rx.try_get() {
            in_failsafe = f;
        }

        if in_failsafe {
            // Fast blink: error pattern
            led.set_low();
            Timer::after_millis(50).await;
            led.set_high();
            Timer::after_millis(50).await;
        } else {
            // Solid ON on valid frames
            led.set_low();
            Timer::after_millis(200).await;
        }
    }
}
