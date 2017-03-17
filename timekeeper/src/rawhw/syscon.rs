#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
#[allow(dead_code)]
pub enum RstN {
    Enabled = 0,
    DeAsserted = 1,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
#[allow(dead_code)]
pub enum FlashOverride {
    MultiCycle = 0,
    SingleCycle = 1,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
#[allow(dead_code)]
pub enum PllClock {
    Irc = 0,
    System = 1,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
#[allow(dead_code)]
pub enum MainClock {
    Irc = 0,
    PllInput = 1,
    Wdt = 2,
    PllOutput = 3,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
#[allow(dead_code)]
pub enum RtcClock {
    ONE_HZ = 0b0000,
    DELAYED_ONE_HZ = 0b0001,
    ONE_KHZ = 0b1010,
    RTC_PCLK = 0b0100,
}

register_block! {
    presetctrl 0x40048004 => {
        0, ssp_rst_n, RstN;
        1, i2c_rst_n, RstN;
        2, uart0_rst_n, RstN;
        3, uart1_rst_n, RstN;
        4, ct16b0_rst_n, RstN;
        5, ct16b1_rst_n, RstN;
        6, ct32b0_rst_n, RstN;
        7, ct32b1_rst_n, RstN;
        8, cmp_rst_n, RstN;
        9, crc_rst_n, RstN;
        10, dma_rst_n, RstN;
        15, flash_override, FlashOverride;
    }
    syspllctrl 0x40048008 => {
        0,4, msel, u8;
        5,6, psel, u8;
    }
    syspllstat 0x4004800C => {
        0, lock, bool;
    }
    syspllclksel 0x40048040 => {
        0,1, sel, PllClock;
    }
    syspllclkuen 0x40048044 => {
        0, ena, bool;
    }
    mainclksel 0x40048070 => {
        0,1, sel, MainClock;
    }
    mainclkuen 0x40048074 => {
        0, ena, bool;
    }
    sysahbclkdiv 0x40048078 => {
        0,7, div, u8;
    }
    rtcclkdiv 0x400480A0 => {
        0,7, div, u8;
    }
    pdruncfg 0x40048238 => {
        0, ircout_pd, bool;
        1, irc_pd, bool;
        2, flash_pd, bool;
        3, bod_pd, bool;
        4, adc_pd, bool;
        5, sysosc_pd, bool;
        6, wdtosc_pd, bool;
        7, syspll_pd, bool;
        15, comp_pd, bool;
    }
    syscfg 0x40038014 => {
        10, wakeuphys, bool;
        11,14, rtcclk, RtcClock;
    }
}
