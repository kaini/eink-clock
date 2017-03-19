// Hardware Map:
// cpu: CPU, PLL, NVIC, 16 Bit Counter 1
// dcf77: GPIO0_11, GPIO0_29, 16 Bit Counter 0
// clock: RTC
// eink: GPIO0_0 - GPIO0_10, GPIO0_14 - GPIO0_19, GPIO0_28
// flash: -
// not usable: GPIO0_12 (BOOTMODE), GPIO0_13 (RESET), GPIO0_25 (SWDIO), GPIO0_26 (SWDCLK), GPIO1_3 (WAKEUP)

pub mod cpu;
pub mod dcf77;
pub mod clock;
pub mod eink;
pub mod flash;
