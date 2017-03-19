use rawhw::ioconfig;
use rawhw::nvic;
use devices::cpu::IsrFlag;
use collections::Vec;

// Signal pin. This is CT16B0_CAP0.
use rawhw::ioconfig::pio0_11 as signal_ioconfig;

// Power pin. This is a high drive pin.
use rawhw::ioconfig::pio0_29 as power_ioconfig;
use rawhw::gpio::gpio0 as power_gpio;
const POWER_BIT: u32 = 29;

// Counter.
use rawhw::counter::ct16b0 as counter;

pub const PAYLOAD_BITS: usize = 58;

static mut COUNTER_FLAG: IsrFlag = IsrFlag::new();

pub unsafe fn init() {
    signal_ioconfig::func::set(3 /* CT16B0_CAP0 */);

    power_ioconfig::mode::set(ioconfig::Pullup::Disabled);
    power_ioconfig::drv::set(true);
    power_gpio::dir::set_bit(POWER_BIT);

    counter::pr::pcval::set(35999);
    counter::ccr::cap0i::set(true);

    disable_module();
}

pub fn receive() -> Vec<bool> {
    debug!("DCF77 receive");
    unsafe { enable_module(); }
    wait_for_module();
    wait_for_signal_start();
    let result = receive_payload();
    unsafe { disable_module(); }
    result
}

unsafe fn enable_module() {
    debug!("DCF77 enable module");
    power_gpio::set::set(1 << POWER_BIT);
    nvic::iser::set(1 << nvic::CT16B0);
    counter::tcr::cen::set(true);
}

unsafe fn disable_module() {
    debug!("DCF77 disable module");
    power_gpio::clr::set(1 << POWER_BIT);
    nvic::icer::set(1 << nvic::CT16B0);
    counter::tcr::cen::set(false);
}

unsafe fn wait_for_raising_edge() -> u32 {
    counter::ccr::cap0re::set(true);
    counter::ccr::cap0fe::set(false);
    COUNTER_FLAG.wait();
    counter::cr0::get()
}

unsafe fn wait_for_falling_edge() -> u32 {
    counter::ccr::cap0re::set(false);
    counter::ccr::cap0fe::set(true);
    COUNTER_FLAG.wait();
    counter::cr0::get()
}

unsafe fn measure_next_low_ms() -> u32 {
    counter::tcr::crst::set(true);
    counter::tcr::crst::set(false);
    let fall_time = wait_for_falling_edge();
    let raise_time = wait_for_raising_edge();
    raise_time - fall_time
}

unsafe fn measure_next_high_ms() -> u32 {
    counter::tcr::crst::set(true);
    counter::tcr::crst::set(false);
    let raise_time = wait_for_raising_edge();
    let fall_time = wait_for_falling_edge();
    fall_time - raise_time
}

fn wait_for_module() {
    debug!("DCF77 wait for module");
    unsafe {
        wait_for_raising_edge();
        wait_for_raising_edge();
        wait_for_raising_edge();
    }
}

fn wait_for_signal_start() {
    debug!("DCF77 wait for signal start");
    while unsafe { measure_next_low_ms() } < 1600 { }
}

fn receive_payload() -> Vec<bool> {
    debug!("DCF77 receive payload");
    let mut result = vec![false; PAYLOAD_BITS];
    for i in 0..PAYLOAD_BITS {
        result[i] = unsafe { measure_next_high_ms() } > 170;
    }
    result
}

pub unsafe extern fn timer16_0_interrupt() {
    if counter::ir::cr0int::get() {
        counter::ir::cr0int::set_fullreg_zero_this_one();
        COUNTER_FLAG.set();
    }
}
