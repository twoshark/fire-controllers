#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use embassy_stm32::usart::{Config as UartConfig, Uart};
use embassy_stm32::wdg::IndependentWatchdog;
use embassy_stm32::{bind_interrupts, dma, peripherals, usart};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::watch::Watch;
use embassy_time::{Instant, Ticker, Timer};

use hotline_protocol::{DecoderEvent, FrameDecoder, StateFrame, POLL_INTERVAL_MS};

use defmt_rtt as _;
use panic_probe as _;

bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
    DMA1_CHANNEL2_3 => dma::InterruptHandler<peripherals::DMA1_CH2>, dma::InterruptHandler<peripherals::DMA1_CH3>;
});

/// Link health status shared between heartbeat_monitor and link_led_driver.
static LINK_HEALTHY: Watch<CriticalSectionRawMutex, bool, 2> = Watch::new();

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

    // -- USART1: full-duplex RS-485 at 115200 (PA9=TX, PA10=RX) with DMA --
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
    let (mut tx, rx) = uart1.split();

    // -- 8 digital input channels from Schmitt frontend (switch closed = HIGH) --
    let ch0 = Input::new(p.PA0, Pull::None);
    let ch1 = Input::new(p.PA1, Pull::None);
    let ch2 = Input::new(p.PA4, Pull::None);
    let ch3 = Input::new(p.PA5, Pull::None);
    let ch4 = Input::new(p.PA6, Pull::None);
    let ch5 = Input::new(p.PA7, Pull::None);
    let ch6 = Input::new(p.PB0, Pull::None);
    let ch7 = Input::new(p.PB1, Pull::None);
    let channels_in = [ch0, ch1, ch2, ch3, ch4, ch5, ch6, ch7];

    // -- Channel LEDs (active LOW) --
    let mut leds: [Output; 8] = [
        Output::new(p.PB2, Level::High, Speed::Low),
        Output::new(p.PA15, Level::High, Speed::Low),
        Output::new(p.PB3, Level::High, Speed::Low),
        Output::new(p.PB4, Level::High, Speed::Low),
        Output::new(p.PB5, Level::High, Speed::Low),
        Output::new(p.PB6, Level::High, Speed::Low),
        Output::new(p.PB7, Level::High, Speed::Low),
        Output::new(p.PB8, Level::High, Speed::Low),
    ];

    // -- Link LED (active LOW): PB9 --
    let led_link = Output::new(p.PB9, Level::High, Speed::Low);

    defmt::info!("input-controller: init complete");

    // Spawn heartbeat monitor (continuous RX in full-duplex mode)
    spawner.spawn(heartbeat_monitor(rx).unwrap());
    // Spawn link LED driver
    spawner.spawn(link_led_driver(led_link).unwrap());

    let mut ticker = Ticker::every(embassy_time::Duration::from_millis(POLL_INTERVAL_MS));
    loop {
        // Read 8 digital channels and pack into protocol bitfield
        let mut channels: u8 = 0;
        for (i, input) in channels_in.iter().enumerate() {
            if input.is_high() {
                channels |= 1 << i;
            }
        }

        // Encode and transmit state frame
        let wire = StateFrame::new(channels).encode();
        if let Err(e) = tx.write(&wire).await {
            defmt::warn!("TX err: {:?}", e);
        }

        // Update channel LEDs (active LOW)
        for (i, led) in leds.iter_mut().enumerate() {
            if channels & (1 << i) != 0 {
                led.set_low();
            } else {
                led.set_high();
            }
        }

        iwdg.pet();
        ticker.next().await;
    }
}

// ---------------------------------------------------------------------------
// Heartbeat monitor task
// ---------------------------------------------------------------------------

type UartRx = embassy_stm32::usart::UartRx<'static, embassy_stm32::mode::Async>;

#[embassy_executor::task]
async fn heartbeat_monitor(mut rx: UartRx) {
    defmt::info!("heartbeat_monitor: started");
    let mut decoder = FrameDecoder::new();
    let mut last_hb = Instant::now();
    let mut buf = [0u8; 1];
    let sender = LINK_HEALTHY.sender();

    loop {
        match embassy_time::with_timeout(
            embassy_time::Duration::from_millis(100),
            rx.read(&mut buf),
        )
        .await
        {
            Ok(Ok(())) => match decoder.feed(buf[0]) {
                DecoderEvent::Heartbeat(hb) => {
                    last_hb = Instant::now();
                    sender.send(true);
                    defmt::debug!(
                        "hb: healthy={} failsafe={}",
                        hb.is_healthy(),
                        hb.is_failsafe_active()
                    );
                }
                DecoderEvent::CrcError => {
                    defmt::warn!("hb: CRC error");
                }
                _ => {}
            },
            Ok(Err(e)) => {
                defmt::warn!("hb RX err: {:?}", e);
            }
            Err(_) => {
                if last_hb.elapsed().as_millis() > 500 {
                    sender.send(false);
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Link LED driver task
// ---------------------------------------------------------------------------

#[embassy_executor::task]
async fn link_led_driver(mut led: Output<'static>) {
    let mut receiver = LINK_HEALTHY.receiver().unwrap();
    let mut healthy = false;

    loop {
        if healthy {
            led.set_low(); // solid ON
                           // Wait for state change or re-check periodically
            if let Ok(h) = embassy_time::with_timeout(
                embassy_time::Duration::from_millis(200),
                receiver.changed(),
            )
            .await
            {
                healthy = h;
            }
        } else {
            // Blink: no heartbeat
            led.set_low();
            Timer::after_millis(100).await;
            led.set_high();
            Timer::after_millis(100).await;
            // Check for state update (non-blocking)
            if let Some(h) = receiver.try_get() {
                healthy = h;
            }
        }
    }
}
