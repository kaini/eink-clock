#![no_std]
#![feature(lang_items)]
#![feature(collections)]
#![feature(start)]
#![feature(asm)]
#![feature(link_args)]
#![feature(const_fn)]
#![cfg_attr(test, allow(dead_code))]

#[cfg_attr(not(test), link_args = "-mthumb -mcpu=cortex-m0 -Tlinker.ld -lc -lgcc")]
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
}

#[link_section = ".isr_vectors"]
#[no_mangle]
pub static ISR_VECTORS: [Option<unsafe extern fn()>; 47] = [
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

    Some(default_handler),  // Wakeup 1
    Some(default_handler),  // Wakeup 2
    Some(default_handler),  // Wakeup 3
    Some(default_handler),  // Wakeup 4
    Some(default_handler),  // Wakeup 5
    Some(default_handler),  // Wakeup 6
    Some(default_handler),  // Wakeup 7
    Some(default_handler),  // Wakeup 8
    Some(default_handler),  // Wakeup 9
    Some(default_handler),  // Wakeup 10
    Some(default_handler),  // Wakeup 11
    Some(default_handler),  // Wakeup 12
    Some(default_handler),  // I2C
    Some(devices::dcf77::timer16_0_interrupt),  // 16 bit timer 0
    Some(default_handler),  // 16 bit timer 1
    Some(default_handler),  // 32 bit timer 0
    Some(default_handler),  // 32 bit timer 1
    Some(default_handler),  // SSP
    Some(default_handler),  // UART0
    Some(default_handler),  // UART1
    Some(default_handler),  // Comperators 0, 1
    Some(default_handler),  // A/D converter
    Some(default_handler),  // Watchdog timer
    Some(default_handler),  // Brown out detect
    None,  // Reserved
    Some(default_handler),  // PIO INT0
    Some(default_handler),  // PIO INT1
    Some(default_handler),  // PIO INT2
    None,  // Reserved
    Some(default_handler),  // DMA
    Some(default_handler),  // RTC
];

#[no_mangle]
pub extern fn reset_handler() {
    extern {
        fn _start();
        static mut __bss_start__: u32;
        static mut __bss_end__: u32;
        static mut __data_start__: u32;
        static mut __data_end__: u32;
        static __data_source_start__: u32;
    }

    unsafe {
        // Disable watchdog timer. This has to happen immediately after startup.
        ptr::write_volatile(0x40004000 as *mut u32, 0x00);
        ptr::write_volatile(0x40004008 as *mut u32, 0xAA);
        ptr::write_volatile(0x40004008 as *mut u32, 0x55);

        // Zero .bss
        let bss_size = (&__bss_end__ as *const u32 as usize) - (&__bss_start__ as *const u32 as usize);
        ptr::write_bytes(&mut __bss_start__, 0, bss_size / 4);

        // Copy .data
        let data_size = (&__data_end__ as *const u32 as usize) - (&__data_start__ as *const u32 as usize);
        ptr::copy_nonoverlapping(&__data_source_start__, &mut __data_start__, data_size / 4);

        // Initialize the C runtime
        _start();
    }

    panic!("Reset handler has quit");
}

#[no_mangle]
pub extern fn nmi_handler() {
    default_handler();
}

#[no_mangle]
pub extern fn hard_fault_handler() {
    default_handler();
}

extern fn default_handler() {
    panic!("Default interrupt handler");
}

#[cfg(not(test))]
#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(msg: core::fmt::Arguments,
                               file: &'static str,
                               line: u32) -> ! {
    debug!("PANIC: {} ({}:{})", msg, file, line);
    breakpoint!();
    loop {}
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
