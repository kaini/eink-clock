use rawhw::syscon;
use rawhw::counter::ct16b1;
use core::ptr::{write_volatile, read_volatile};

pub unsafe fn early_init() {
    // Clear the deep power down flag
    syscon::pcon::dpdflag::set_fullreg_zero_this_one();

    // See Section 4.10.4.1 on how to calculate these values.
    // The chosen values result in a PLL output clock of 36 MHz.
    syscon::syspllctrl::msel::set(2);
    syscon::syspllctrl::psel::set(2);
    syscon::syspllclksel::sel::set(syscon::PllClock::Irc);
    syscon::syspllclkuen::ena::set(false);
    syscon::syspllclkuen::ena::set(true);
    syscon::pdruncfg::syspll_pd::set(false);
    while !syscon::syspllstat::lock::get() { }

    // Set the CPU to 36 MHz as well
    syscon::presetctrl::flash_override::set(syscon::FlashOverride::MultiCycle);
    syscon::sysahbclkdiv::div::set(1);
    syscon::mainclksel::sel::set(syscon::MainClock::PllOutput);
    syscon::mainclkuen::ena::set(false);
    syscon::mainclkuen::ena::set(true);
}

pub fn usleep(us: u16) {
    unsafe {
        ct16b1::pr::pcval::set(35);  // Divide by 36 -> one count per us
        ct16b1::tcr::cen::set(true);
        ct16b1::tcr::crst::set(true);
        ct16b1::tcr::crst::set(false);
        while ct16b1::tc::tc::get() < us { }
        ct16b1::tcr::cen::set(false);
    }
}

pub fn get_data(index: usize) -> u32 {
    unsafe {
        match index {
            0 => syscon::gpdata0::get(),
            1 => syscon::gpdata1::get(),
            2 => syscon::gpdata2::get(),
            3 => syscon::gpdata3::get(),
            _ => panic!("Index out of bounds")
        }
    }
}

#[cfg(all(not(test), not(feature = "stay_awake")))]
pub fn deep_power_down(data: &[u32]) -> ! {
    use rawhw::nvic;

    assert!(data.len() <= 4);
    unsafe {
        syscon::pcon::dpden::set(true);
        if data.len() >= 1 { syscon::gpdata0::set(data[0]); }
        if data.len() >= 2 { syscon::gpdata1::set(data[1]); }
        if data.len() >= 3 { syscon::gpdata2::set(data[2]); }
        if data.len() >= 4 { syscon::gpdata3::set(data[3]); }
        nvic::scr::sleepdeep::set(true);
        syscon::pdruncfg::ircout_pd::set(false);
        syscon::pdruncfg::irc_pd::set(false);
        asm!("wfi" :::: "volatile");
    }
    unreachable!();
}

#[cfg(any(test, feature = "stay_awake"))]
pub fn deep_power_down(_data: &[u32]) -> ! {
    panic!("cpu::deep_power_down called");
}

pub struct IsrFlag {
    flag: bool,
}

impl IsrFlag {
    pub const fn new() -> IsrFlag {
        IsrFlag {
            flag: false,
        }
    }

    #[cfg(not(test))]
    pub unsafe fn wait(&'static mut self) {
        let mut current_flag = false;
        while !current_flag {
            let primask: u32;
            asm!(
                "mrs $0, primask; cpsid i;"
                : "=r"(primask)  // Outputs
                :  // Inputs
                : "primask"  // Clobbers
                : "volatile"
            );
            current_flag = read_volatile(&mut self.flag);
            if current_flag {
                write_volatile(&mut self.flag, false);
            } else {
                asm!("wfi" :::: "volatile");
            }
            asm!(
                "msr primask, $0; isb;"
                :  // Outputs
                : "r"(primask)  // Inputs
                : "primask"  // Clobbers
                : "volatile"
            );
        }
    }

    #[cfg(test)]
    pub unsafe fn wait(&'static mut self) {
        unimplemented!();
    }

    #[cfg(not(test))]
    pub unsafe fn set(&'static mut self) {
        write_volatile(&mut self.flag, true);
    }

    #[cfg(test)]
    pub unsafe fn set(&'static mut self) {
        unimplemented!();
    }
}
