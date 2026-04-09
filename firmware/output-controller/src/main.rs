#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use embassy_stm32::mode::Async;
use embassy_stm32::usart::{Config as UartConfig, Uart, UartRx, UartTx};
use embassy_stm32::wdg::IndependentWatchdog;
use embassy_stm32::{bind_interrupts, dma, peripherals, usart};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::watch::Watch;
use embassy_time::{Instant, Ticker, Timer};

use hotline_protocol::{
    DecoderEvent, FrameDecoder, HeartbeatFrame, COMM_WATCHDOG_TIMEOUT_MS, HEARTBEAT_INTERVAL_MS,
    POLL_INTERVAL_MS,
};

use defmt_rtt as _;
use panic_probe as _;

bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
    DMA1_CHANNEL2_3 => dma::InterruptHandler<peripherals::DMA1_CH2>, dma::InterruptHandler<peripherals::DMA1_CH3>;
});

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

    // -- 8 override digital inputs from Schmitt frontend (switch closed = HIGH) --
    let ovr0 = Input::new(p.PA0, Pull::None);
    let ovr1 = Input::new(p.PA1, Pull::None);
    let ovr2 = Input::new(p.PA4, Pull::None);
    let ovr3 = Input::new(p.PA5, Pull::None);
    let ovr4 = Input::new(p.PA6, Pull::None);
    let ovr5 = Input::new(p.PA7, Pull::None);
    let ovr6 = Input::new(p.PB0, Pull::None);
    let ovr7 = Input::new(p.PB1, Pull::None);
    let overrides = [ovr0, ovr1, ovr2, ovr3, ovr4, ovr5, ovr6, ovr7];

    // -- MOSFET gate outputs (default LOW = off, reinforced by 10k pull-downs in HW) --
    let mut gates: [Output; 8] = [
        Output::new(p.PB2, Level::Low, Speed::Low),  // CH0
        Output::new(p.PB10, Level::Low, Speed::Low), // CH1
        Output::new(p.PB11, Level::Low, Speed::Low), // CH2
        Output::new(p.PB12, Level::Low, Speed::Low), // CH3
        Output::new(p.PB13, Level::Low, Speed::Low), // CH4
        Output::new(p.PA8, Level::Low, Speed::Low),  // CH5 (moved off PA11 for USB)
        Output::new(p.PA15, Level::Low, Speed::Low), // CH6
        Output::new(p.PB3, Level::Low, Speed::Low),  // CH7
    ];

    // -- Channel LEDs (active LOW) --
    let mut ch_leds: [Output; 8] = [
        Output::new(p.PB4, Level::High, Speed::Low), // CH0
        Output::new(p.PB5, Level::High, Speed::Low), // CH1
        Output::new(p.PB6, Level::High, Speed::Low), // CH2
        Output::new(p.PB7, Level::High, Speed::Low), // CH3
        Output::new(p.PB8, Level::High, Speed::Low), // CH4
        Output::new(p.PB9, Level::High, Speed::Low), // CH5
        Output::new(p.PC6, Level::High, Speed::Low), // CH6
        Output::new(p.PC7, Level::High, Speed::Low), // CH7
    ];

    // -- Link LED (active LOW): PC14 --
    let led_link = Output::new(p.PC14, Level::High, Speed::Low);

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

    let mut ticker = Ticker::every(embassy_time::Duration::from_millis(POLL_INTERVAL_MS));
    let mut serial_rx = SERIAL_STATE.receiver().unwrap();
    let mut failsafe_rx = FAILSAFE_ACTIVE.receiver().unwrap();

    loop {
        // Read 8 override digital inputs.
        let mut override_on = [false; 8];
        for (i, input) in overrides.iter().enumerate() {
            override_on[i] = input.is_high();
        }

        // Get latest serial state and failsafe status
        let serial_channels = serial_rx.try_get().unwrap_or(0);
        let failsafe = failsafe_rx.try_get().unwrap_or(false);

        // Merge: override takes precedence; failsafe forces all OFF
        for i in 0..8u8 {
            let on = if failsafe {
                false
            } else {
                // Closed override switch forces channel ON; open means serial control.
                if override_on[i as usize] {
                    true
                } else {
                    serial_channels & (1 << i) != 0
                }
            };

            if on {
                gates[i as usize].set_high();
                ch_leds[i as usize].set_low(); // LED ON
            } else {
                gates[i as usize].set_low();
                ch_leds[i as usize].set_high(); // LED OFF
            }
        }

        iwdg.pet();
        ticker.next().await;
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
