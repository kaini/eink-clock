#![no_std]
#![feature(lang_items)]
#![feature(collections)]
#![feature(asm)]
#![cfg_attr(test, allow(dead_code))]

extern crate my_allocator;
extern crate collections;

#[macro_use]
mod debug;
mod rawhw;
mod devices;
mod app;

use devices::cpu::Cpu;
use devices::dcf77::Dcf77;
use devices::clock::Clock;
use app::datetime::Datetime;

// Export interrupt handlers (yes this is ugly but whatever)
pub use devices::dcf77::TIMER16_0_IRQHandler;

#[no_mangle]
pub extern fn rust_main() -> ! {
    let _cpu = unsafe { Cpu::new() };
    let mut dcf77 = unsafe { Dcf77::new() };
    let clock = unsafe { Clock::new() };

    //let signal = &dcf77.receive();
    //let basetime = Basetime::new(signal);
    //debug!("{:?}", basetime);

    loop {
        debug!("{}", clock.current_time());
    }

    panic!("The main function has quit!");
}

#[cfg(not(test))]
#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(msg: core::fmt::Arguments,
                               file: &'static str,
                               line: u32) -> ! {
    let s = collections::fmt::format(format_args!("PANIC: {} ({}:{})", msg, file, line));
    debug::writeln(s.as_str());
    breakpoint!();
    loop { }
}

#[cfg(test)]
extern {
    fn putchar(chr: i32) -> i32;
}

#[cfg(test)]
#[no_mangle]
pub extern fn print_string(ptr: *const u8, size: usize) {
    for i in 0..size {
        unsafe {
            putchar(*ptr.offset(i as isize) as i32);
        }
    }
}
