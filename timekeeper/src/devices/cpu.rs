use rawhw::syscon;

pub struct Cpu {
    _phantom: ()
}

impl Cpu {
    pub unsafe fn new() -> Cpu {
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

        Cpu { _phantom: () }
    }
}
