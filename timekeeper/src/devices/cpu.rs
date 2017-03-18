use rawhw::syscon;
use rawhw::counter::ct16b1;
use core::sync::atomic::{fence, Ordering};
use core::ops::{Deref, DerefMut};

pub unsafe fn init() {
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

pub struct CriticalSection {
    primask: u32,
}

impl CriticalSection {
    pub fn begin() -> CriticalSection {
        let mut primask: u32;
        unsafe {
            asm!(
                "mrs $0, primask; cpsid i;"
                : "=r"(primask) // Outputs
                :  // Inputs
                : "primask"  // Clobbers
                : "volatile"
            );
        }
        CriticalSection {
            primask: primask,
        }
    }
}

impl Drop for CriticalSection {
    fn drop(&mut self) {
        unsafe {
            asm!(
                "msr primask, $0"
                :  // Outputs
                : "r"(self.primask)  // Inputs
                : "primask"  // Clobbers
                : "volatile"
            );
        }
    }
}

pub struct IsrMutex<T> {
    value: T,
}

pub struct IsrMutexGuard<'a, T: 'a> {
    value: &'a mut T,
    cs: CriticalSection,
}

impl<T> IsrMutex<T> {
    pub const fn new(value: T) -> IsrMutex<T> {
        IsrMutex {
            value: value,
        }
    }

    pub fn lock<'a>(&'a mut self) -> IsrMutexGuard<'a, T> {
        let cs = CriticalSection::begin();
        fence(Ordering::Acquire);
        IsrMutexGuard {
            value: &mut self.value,
            cs: cs,
        }
    }
}

impl<'a, T> Drop for IsrMutexGuard<'a, T> {
    fn drop(&mut self) {
        fence(Ordering::Release);
    }
}

impl<'a, T> Deref for IsrMutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.value
    }
}

impl<'a, T> DerefMut for IsrMutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.value
    }
}
