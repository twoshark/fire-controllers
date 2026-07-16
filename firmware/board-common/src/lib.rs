#![no_std]

//! Shared STM32G0B1 bring-up helpers for the input and output controllers.

use embassy_stm32::usart::Config as UartConfig;
use embassy_stm32::Config;

/// Hotline RS-485 UART baud rate.
pub const RS485_BAUD: u32 = 115_200;

/// Independent watchdog timeout (microseconds).
pub const IWDG_TIMEOUT_US: u32 = 1_000_000;

/// RX scratch buffer for `read_until_idle` bursts (several Hotline frames).
pub const UART_RX_BUF_LEN: usize = 32;

/// SYSCLK = 64 MHz from HSI via PLL (HSI 16 MHz × 8 / 2).
pub fn clock_config() -> Config {
    let mut config = Config::default();
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
    config
}

/// USART config for full-duplex Hotline at [`RS485_BAUD`].
pub fn usart_config() -> UartConfig {
    let mut config = UartConfig::default();
    config.baudrate = RS485_BAUD;
    config
}
