use rawhw::ioconfig;
use rawhw::nvic;
use core::ptr::{write_volatile, read_volatile};

// Signal pin. This is CT16B0_CAP0.
use rawhw::ioconfig::pio0_11 as signal_ioconfig;

// Power pin. This is a high drive pin.
use rawhw::ioconfig::pio0_29 as power_ioconfig;
use rawhw::gpio::gpio0 as power_gpio;
const POWER_BIT: u32 = 29;

// Counter.
use rawhw::counter::ct16b0 as counter;

pub const PAYLOAD_BITS: usize = 58;

// TODO refactor and make a atomic struct
static mut INTERRUPT_HAPPENED: bool = false;

pub struct Dcf77 {
    _phantom: ()
}

impl Dcf77 {
    pub unsafe fn new() -> Dcf77 {
        debug_trace!();

        signal_ioconfig::func::set(3 /* CT16B0_CAP0 */);

        power_ioconfig::mode::set(ioconfig::Pullup::Disabled);
        power_ioconfig::drv::set(true);
        power_gpio::dir::set_bit(POWER_BIT);

        counter::pr::pcval::set(35999);
        counter::ccr::cap0i::set(true);

        let mut dcf77 = Dcf77 { _phantom: () };
        dcf77.disable_module();
        dcf77
    }

    pub fn receive(&mut self) -> [bool; PAYLOAD_BITS] {
        debug_trace!();
        unsafe { self.enable_module(); }
        self.wait_for_module();
        self.wait_for_signal_start();
        let result = self.receive_payload();
        unsafe { self.disable_module(); }
        result
    }

    unsafe fn enable_module(&mut self) {
        debug_trace!();
        power_gpio::set::set(1 << POWER_BIT);
        nvic::iser::set(1 << nvic::CT16B0);
        counter::tcr::cen::set(true);
    }

    unsafe fn disable_module(&mut self) {
        debug_trace!();
        power_gpio::clr::set(1 << POWER_BIT);
        nvic::icer::set(1 << nvic::CT16B0);
        counter::tcr::cen::set(false);
    }

    unsafe fn wait_for_interrupt(&mut self) {
        write_volatile(&mut INTERRUPT_HAPPENED, false);
        while !read_volatile(&mut INTERRUPT_HAPPENED) { }
    }

    unsafe fn wait_for_raising_edge(&mut self) -> u16 {
        counter::ccr::cap0re::set(true);
        counter::ccr::cap0fe::set(false);
        self.wait_for_interrupt();
        counter::cr0::cap::get()
    }

    unsafe fn wait_for_falling_edge(&mut self) -> u16 {
        counter::ccr::cap0re::set(false);
        counter::ccr::cap0fe::set(true);
        self.wait_for_interrupt();
        counter::cr0::cap::get()
    }

    unsafe fn measure_next_low_ms(&mut self) -> u16 {
        counter::tcr::crst::set(true);
        counter::tcr::crst::set(false);
        let fall_time = self.wait_for_falling_edge();
        let raise_time = self.wait_for_raising_edge();
        raise_time - fall_time
    }

    unsafe fn measure_next_high_ms(&mut self) -> u16 {
        counter::tcr::crst::set(true);
        counter::tcr::crst::set(false);
        let raise_time = self.wait_for_raising_edge();
        let fall_time = self.wait_for_falling_edge();
        fall_time - raise_time
    }

    fn wait_for_module(&mut self) {
        debug_trace!();
        unsafe {
            self.wait_for_raising_edge();
            self.wait_for_raising_edge();
            self.wait_for_raising_edge();
        }
    }

    fn wait_for_signal_start(&mut self) {
        debug_trace!();
        while unsafe { self.measure_next_low_ms() } < 1600 { }
    }

    fn receive_payload(&mut self) -> [bool; PAYLOAD_BITS] {
        debug_trace!();
        let mut result = [false; PAYLOAD_BITS];
        for i in 0..PAYLOAD_BITS {
            result[i] = unsafe { self.measure_next_high_ms() } > 170;
        }
        result
    }
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern fn TIMER16_0_IRQHandler() {
    if counter::ir::cr0int::get() {
        counter::ir::cr0int::set_fullreg_zero_this_one();
        write_volatile(&mut INTERRUPT_HAPPENED, true);
    }
}
