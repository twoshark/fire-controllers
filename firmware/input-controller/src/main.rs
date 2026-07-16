#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_futures::select::{select3, select_array, Either3};
use embassy_stm32::exti::{ExtiInput, InterruptHandler as ExtiInterruptHandler};
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use embassy_stm32::usart::{Config as UartConfig, Uart};
use embassy_stm32::wdg::IndependentWatchdog;
use embassy_stm32::{bind_interrupts, dma, interrupt, peripherals, usart};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::watch::Watch;
use embassy_time::{Duration, Instant, Ticker, Timer};

use hotline_protocol::{
    DecoderEvent, FrameDecoder, StateFrame, HEARTBEAT_LOSS_TIMEOUT_MS, STATE_KEEPALIVE_MS,
};

use defmt_rtt as _;
use panic_probe as _;

// Pin map: hardware/v1.0.0/PIN_MAP.md
//   IN CH0..CH5 -> PA0, PA1, PA4, PA5, PA6, PA7 (EXTI)
//   IN CH6..CH7 -> PB0, PB1 (polled; EXTI0/1 clash with PA0/PA1)
//   USART1 TX/RX -> PA9 / PA10 (U2A.DI / U2B.RO)
//   CH LEDs LED3..10 -> PB10, PA15, PB3..PB8 (active-low)
//   LINK LED2 -> PB9 (active-low)
//   USB PA11/PA12 ; SWD PA13/PA14 ; BOOT0=SW2, NRST=SW1

/// Poll period for CH6/CH7 (PB0/PB1 cannot share EXTI0/1 with PA0/PA1).
const POLLED_INPUT_MS: u64 = 1;

bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
    DMA1_CHANNEL2_3 => dma::InterruptHandler<peripherals::DMA1_CH2>, dma::InterruptHandler<peripherals::DMA1_CH3>;
    EXTI0_1 => ExtiInterruptHandler<interrupt::typelevel::EXTI0_1>;
    EXTI4_15 => ExtiInterruptHandler<interrupt::typelevel::EXTI4_15>;
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

    // -- Digital inputs (bit n = channel n) --
    // CH0..CH5 use EXTI. CH6/CH7 are on PB0/PB1 which share EXTI0/1 with PA0/PA1,
    // so they are polled at POLLED_INPUT_MS.
    let mut exti_in = [
        ExtiInput::new(p.PA0, p.EXTI0, Pull::None, Irqs), // CH0 / J2a.1
        ExtiInput::new(p.PA1, p.EXTI1, Pull::None, Irqs), // CH1 / J2a.2
        ExtiInput::new(p.PA4, p.EXTI4, Pull::None, Irqs), // CH2 / J2a.3
        ExtiInput::new(p.PA5, p.EXTI5, Pull::None, Irqs), // CH3 / J2a.4
        ExtiInput::new(p.PA6, p.EXTI6, Pull::None, Irqs), // CH4 / J2b.1
        ExtiInput::new(p.PA7, p.EXTI7, Pull::None, Irqs), // CH5 / J2b.2
    ];
    let ch6 = Input::new(p.PB0, Pull::None); // CH6 / J2b.3
    let ch7 = Input::new(p.PB1, Pull::None); // CH7 / J2b.4

    // -- Channel LEDs LED3..LED10 (active LOW) --
    let mut leds: [Output; 8] = [
        Output::new(p.PB10, Level::High, Speed::Low), // LED3  CH0
        Output::new(p.PA15, Level::High, Speed::Low), // LED4  CH1
        Output::new(p.PB3, Level::High, Speed::Low),  // LED5  CH2
        Output::new(p.PB4, Level::High, Speed::Low),  // LED6  CH3
        Output::new(p.PB5, Level::High, Speed::Low),  // LED7  CH4
        Output::new(p.PB6, Level::High, Speed::Low),  // LED8  CH5
        Output::new(p.PB7, Level::High, Speed::Low),  // LED9  CH6
        Output::new(p.PB8, Level::High, Speed::Low),  // LED10 CH7
    ];

    // -- Link LED2 (active LOW): PB9 --
    let led_link = Output::new(p.PB9, Level::High, Speed::Low);

    defmt::info!("input-controller: init complete");

    spawner.spawn(heartbeat_monitor(rx).unwrap());
    spawner.spawn(link_led_driver(led_link).unwrap());

    let mut keepalive = Ticker::every(Duration::from_millis(STATE_KEEPALIVE_MS));
    let mut poll = Ticker::every(Duration::from_millis(POLLED_INPUT_MS));
    let mut last_channels: u8 = 0xFF; // force first TX
    let mut last_tx = Instant::now();

    loop {
        let mut channels: u8 = 0;
        for (i, input) in exti_in.iter().enumerate() {
            if input.is_high() {
                channels |= 1 << i;
            }
        }
        if ch6.is_high() {
            channels |= 1 << 6;
        }
        if ch7.is_high() {
            channels |= 1 << 7;
        }

        let changed = channels != last_channels;
        let keepalive_due = last_tx.elapsed().as_millis() >= STATE_KEEPALIVE_MS;
        if changed || keepalive_due {
            let wire = StateFrame::new(channels).encode();
            if let Err(e) = tx.write(&wire).await {
                defmt::warn!("TX err: {:?}", e);
            }
            last_channels = channels;
            last_tx = Instant::now();

            for (i, led) in leds.iter_mut().enumerate() {
                if channels & (1 << i) != 0 {
                    led.set_low();
                } else {
                    led.set_high();
                }
            }
        }

        iwdg.pet();

        let edges = exti_in.each_mut().map(|c| c.wait_for_any_edge());
        match select3(select_array(edges), poll.next(), keepalive.next()).await {
            Either3::First(_) => {}  // CH0..CH5 edge
            Either3::Second(_) => {} // CH6/CH7 poll tick
            Either3::Third(_) => {}  // keepalive bound
        }
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
    let mut last_hb: Option<Instant> = None;
    let mut buf = [0u8; 1];
    let sender = LINK_HEALTHY.sender();
    let mut link_healthy = false;

    loop {
        match embassy_time::with_timeout(
            embassy_time::Duration::from_millis(100),
            rx.read(&mut buf),
        )
        .await
        {
            Ok(Ok(())) => match decoder.feed(buf[0]) {
                DecoderEvent::Heartbeat(hb) => {
                    last_hb = Some(Instant::now());
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
            Err(_) => {}
        }

        // Evaluate heartbeat age every pass so link health updates even when RX
        // keeps returning non-heartbeat bytes or transient UART errors.
        let now_healthy = last_hb
            .map(|ts| ts.elapsed().as_millis() <= HEARTBEAT_LOSS_TIMEOUT_MS)
            .unwrap_or(false);
        if now_healthy != link_healthy {
            link_healthy = now_healthy;
            sender.send(link_healthy);
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
