#![no_std]
#![feature(lang_items)]
#![feature(collections)]
#![feature(start)]
#![feature(asm)]
#![feature(link_args)]
#![feature(const_fn)]
#![cfg_attr(test, allow(dead_code))]

#[link_args = "-mthumb -mcpu=cortex-m0 -Tlinker.ld -lc -lgcc"]
extern {}

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
use core::ptr;
use core::mem::transmute;

// Export interrupt handlers (yes this is ugly but whatever)
pub use devices::dcf77::TIMER16_0_IRQHandler;

#[start]
pub fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let _cpu = unsafe { Cpu::new() };
    let mut dcf77 = unsafe { Dcf77::new() };
    let clock = unsafe { Clock::new() };

    panic!("The main function has quit!")
}

extern {
    fn __stack_end__();
    fn __checksum__();
    fn _start();
}

#[link_section = ".isr_vectors"]
#[no_mangle]
pub static ISR_VECTORS: [Option<unsafe extern fn()>; 16] = [
    Some(__stack_end__),  // Initial SP
    Some(reset_handler),  // Reset handler
    Some(nmi_handler),  // NMI handler
    Some(hard_fault_handler),  // Hard fault handler
    None,  // Reserved
    None,  // Reserved
    None,  // Reserved
    Some(__checksum__),  // Checksum
    None,  // Reserved
    None,  // Reserved
    None,  // Reserved
    Some(default_handler),  // SVC handler
    None,  // Reserved
    None,  // Reserved
    Some(default_handler),  // PendSV handler
    Some(default_handler),  // SysTick handler
];

#[no_mangle]
pub extern fn reset_handler() {
    unsafe {
        _start();
    }
}

#[no_mangle]
pub extern fn nmi_handler() {
    default_handler();
}

#[no_mangle]
pub extern fn hard_fault_handler() {
    default_handler();
}

#[no_mangle]
pub extern fn default_handler() {
    panic!("Default ISR handler!");
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

#[no_mangle]
pub extern fn print_string(_ptr: *const u8, _size: usize) {
}

#[no_mangle]
pub extern fn _exit() -> ! {
    loop {}
}

#[no_mangle]
pub extern fn abort() -> ! {
    loop {}
}

#[no_mangle]
pub extern fn _sbrk(incr: i32) -> *mut u8 {
    extern {
        static mut __heap_end__: u8;
        static mut __heap_start__: u8;
    }

    static mut HEAP_AT: *mut u8 = ptr::null_mut();
    unsafe {
        if HEAP_AT == ptr::null_mut() {
            HEAP_AT = &mut __heap_start__;
        }
        let prev_heap_at = HEAP_AT;
        HEAP_AT = HEAP_AT.offset(incr as isize);
        if HEAP_AT < &mut __heap_end__ {
            prev_heap_at
        } else {
            panic!("Out of heap!");
        }
    }
}
