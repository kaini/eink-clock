use rawhw::ioconfig;
use rawhw::nvic;
use devices::cpu::IsrMutex;
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

#[derive(Debug)]
struct IsrData {
    payload: &'static mut [bool],
    state: State,
}
static mut SHARED: IsrMutex<IsrData> = IsrMutex::new(IsrData { payload: &mut [false; PAYLOAD_BITS], state: State::Idle });

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Edge {
    Raising,
    Falling,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum State {
    Idle,
    WaitForModule(u8),
    WaitForSignal(Edge, u32),
    Receive(Edge, usize),
}

pub unsafe fn init() {
    debug!("Init DCF77");

    signal_ioconfig::func::set(3 /* CT16B0_CAP0 */);

    power_ioconfig::mode::set(ioconfig::Pullup::Disabled);
    power_ioconfig::drv::set(true);
    power_gpio::dir::set_bit(POWER_BIT);

    counter::pr::pcval::set(35999);
    counter::ccr::cap0i::set(true);

    disable_module();
}

pub fn start_receive() {
    debug!("Receive DCF77");
    unsafe {
        let mut shared = SHARED.lock();
        if shared.state == State::Idle {
            for bit in shared.payload.iter_mut() {
                *bit = false;
            }
            shared.state = State::WaitForModule(3);
            wait_for_raising_edge();
            enable_module();
        }
    }
}

pub fn get_result() -> Option<Vec<bool>> {
    unsafe {
        let shared = SHARED.lock();
        if shared.state != State::Idle {
            None
        } else {
            disable_module();
            Some(shared.payload.to_vec())
        }
    }
}

unsafe fn enable_module() {
    debug!("Enable DCF77");
    power_gpio::set::set(1 << POWER_BIT);
    nvic::iser::set(1 << nvic::CT16B0);
    counter::tcr::cen::set(true);
}

unsafe fn disable_module() {
    debug!("Disable DCF77");
    power_gpio::clr::set(1 << POWER_BIT);
    nvic::icer::set(1 << nvic::CT16B0);
    counter::tcr::cen::set(false);
}

unsafe fn wait_for_raising_edge() {
    counter::ccr::cap0re::set(true);
    counter::ccr::cap0fe::set(false);
}

unsafe fn wait_for_falling_edge() {
    counter::ccr::cap0re::set(false);
    counter::ccr::cap0fe::set(true);
}

unsafe fn reset_counter() {
    counter::tcr::crst::set(true);
    counter::tcr::crst::set(false);
}

unsafe fn read_counter() -> u32 {
    counter::cr0::get()
}

pub unsafe extern fn timer16_0_interrupt() {
    if counter::ir::cr0int::get() {
        counter::ir::cr0int::set_fullreg_zero_this_one();

        let mut shared = SHARED.lock();
        shared.state = match shared.state {
            State::WaitForModule(0) => {
                wait_for_falling_edge();
                State::WaitForSignal(Edge::Falling, 0)
            },
            State::WaitForModule(todo) => {
                wait_for_raising_edge();
                State::WaitForModule(todo - 1)
            },
            State::WaitForSignal(Edge::Falling, time) => {
                reset_counter();
                wait_for_raising_edge();
                State::WaitForSignal(Edge::Raising, 0)
            },
            State::WaitForSignal(Edge::Raising, time) => {
                let low_time = read_counter();
                if low_time < 1600 {
                    wait_for_falling_edge();
                    State::WaitForSignal(Edge::Falling, low_time)
                } else {
                    wait_for_raising_edge();
                    State::Receive(Edge::Raising, 0)
                }
            },
            State::Receive(Edge::Raising, at) => {
                reset_counter();
                wait_for_falling_edge();
                State::Receive(Edge::Falling, at)
            },
            State::Receive(Edge::Falling, at) => {
                let high_time = read_counter();
                shared.payload[at] = high_time > 170;
                if at == PAYLOAD_BITS - 1 {
                    State::Idle
                } else {
                    wait_for_raising_edge();
                    State::Receive(Edge::Raising, at + 1)
                }
            },
            State::Idle => {
                State::Idle
            },
        }
    }
}
