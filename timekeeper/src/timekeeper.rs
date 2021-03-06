#![no_std]
#![feature(lang_items)]
#![feature(collections)]
#![feature(alloc)]
#![feature(start)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(core_intrinsics)]
#![allow(dead_code)]
#![cfg_attr(not(test), no_builtins)]

#[cfg(not(test))]
extern crate my_allocator;
#[macro_use]
extern crate collections;
extern crate alloc;
#[cfg(not(test))]
extern crate rlibc;

#[macro_use]
mod debug;
mod rawhw;
mod devices;
mod app;

use devices::{cpu, eink, flash, dcf77, clock, ram};
use app::datetime::Datetime;
use app::graphics;
use app::graphics::HorizontalAlign;
use app::eutime::central_localtime;
use core::ptr;
use core::f32::consts::PI;
use core::intrinsics::{sinf32, cosf32, roundf32};

const RESYNC_TIME: i32 = 7 * 24 * 60 * 60;  // 7 days
const WEEKDAYS: [&str; 7] = ["SO", "MO", "DI", "MI", "DO", "FR", "SA"];

unsafe fn hardware_init() {
    extern {
        static mut __bss_start__: u32;
        static mut __bss_end__: u32;
        static mut __data_start__: u32;
        static mut __data_end__: u32;
        static __data_source_start__: u32;
    }

    // Disable watchdog timer. This has to happen immediately after startup.
    ptr::write_volatile(0x40004000 as *mut u32, 0x00);
    ptr::write_volatile(0x40004008 as *mut u32, 0xAA);
    ptr::write_volatile(0x40004008 as *mut u32, 0x55);

    // Early init of critical hardware
    eink::early_init();
    cpu::early_init();

    // Zero .bss
    let bss_size = (&__bss_end__ as *const u32 as usize) - (&__bss_start__ as *const u32 as usize);
    ptr::write_bytes(&mut __bss_start__, 0, bss_size / 4);

    // Copy .data
    let data_size = (&__data_end__ as *const u32 as usize) - (&__data_start__ as *const u32 as usize);
    ptr::copy_nonoverlapping(&__data_source_start__, &mut __data_start__, data_size / 4);

    // Init the allocator
    my_allocator::init();

    // Init other hardware
    eink::init();
    dcf77::init();
    clock::init();
    ram::init();
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    unsafe { hardware_init(); }

    let set_pixel = &|x, y| {
        if 0 <= x && x < 600 && 0 <= y && y < 800 {
            ram::set(599 - x as usize, y as usize);
        }
    };
    
    let zero_time = if let Ok(zt) = Datetime::from_bits(cpu::get_data(0) as u64 | ((cpu::get_data(1) as u64) << 32)) {
        zt
    } else {
        ram::zero();
        graphics::render_triangle(set_pixel, 200, 400, 400, 400, 300, 300);
        graphics::render_triangle(set_pixel, 200, 400, 400, 400, 300, 500);
        eink::render(&|scanline, buffer| {
            ram::get_column(scanline, buffer);
        });
        adjust_time()
    };
    let now_s = clock::current_time();
    let now = { let mut now = zero_time.clone(); now.offset_seconds(now_s); central_localtime(&now) };

    let status_line = format!("RTC: {}    LAST SYNC: {}.{}.{} {:02}:{:02}",
        now_s,
        zero_time.day(), zero_time.month(), zero_time.year(),
        zero_time.hour(), zero_time.minute());
    ram::zero();
    graphics::render_image(
        set_pixel,
        flash::CLOCK, flash::CLOCK_W, flash::CLOCK_H,
        0, 0,
        0, 0, flash::CLOCK_W, flash::CLOCK_H);
    graphics::render_text(set_pixel, &status_line, &flash::SMALL_FONT, 597, 775, HorizontalAlign::RIGHT);
    graphics::render_text(
        set_pixel,
        &format!("{} {}.{}.", WEEKDAYS[now.weekday() as usize], now.day(), now.month()),
        &flash::LARGE_FONT, 300, 600, HorizontalAlign::CENTER);
    let minute_angle = -PI / 30.0 * now.minute() as f32 - PI / 2.0;
    let minute_x = 300 + round(-cos(minute_angle) * 280.0) as i32;
    let minute_y = 300 + round(sin(minute_angle) * 280.0) as i32;
    graphics::render_line(set_pixel, 300, 300, minute_x, minute_y, 20);
    let hour_angle = -PI / 360.0 * ((now.hour() % 12) * 60 + now.minute()) as f32 - PI / 2.0;
    let hour_x = 300 + round(-cos(hour_angle) * 200.0) as i32;
    let hour_y = 300 + round(sin(hour_angle) * 200.0) as i32;
    graphics::render_line(set_pixel, 300, 300, hour_x, hour_y, 30);

    eink::render(&|scanline, buffer| {
        ram::get_column(scanline, buffer);
    });

    clock::set_interrupt_time(((now_s / 60) + 1) * 60);
    let now_serialized = if now_s > RESYNC_TIME && now.hour() == 4 {
        0
    } else {
        zero_time.to_bits()
    };
    cpu::deep_power_down(&[
        (now_serialized & 0xFFFFFFFF) as u32,
        (now_serialized >> 32) as u32,
    ]);
}

/// Receives the time and returns the new zero time.
#[cfg(not(feature = "fake_time"))]
fn adjust_time() -> Datetime {
    loop {
        let payload = dcf77::receive();
        match Datetime::from_dcf77(&payload) {
            Ok(new_zero_time) => {
                clock::reset_time();
                return new_zero_time;
            },
            Err(error) => {
                debug!("DCF parse error: {}", error);
            }
        }
    }
}

#[cfg(feature = "fake_time")]
fn adjust_time() -> Datetime {
    clock::reset_time();
    return Datetime::new(2000, 1, 1, 15, 37, 0, 3600).unwrap();
}

#[cfg(not(test))]
extern {
    fn __stack_end__();
    fn __checksum__();
}

#[cfg(not(test))]
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
    Some(devices::clock::rtc_interrupt),  // RTC
];

#[cfg(not(test))]
#[no_mangle]
pub extern fn reset_handler() {
    main(0, ptr::null());
    panic!("Reset handler has quit");
}

#[cfg(not(test))]
#[no_mangle]
pub extern fn nmi_handler() {
    default_handler();
}

#[cfg(not(test))]
#[no_mangle]
pub extern fn hard_fault_handler() {
    default_handler();
}

#[cfg(not(test))]
extern fn default_handler() {
    loop {}
}

#[cfg(not(test))]
#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(msg: core::fmt::Arguments,
                               file: &'static str,
                               line: u32) -> ! {
    debug!("PANIC: {} ({}:{})", msg, file, line);
    loop {}
}

#[cfg(not(test))]
#[no_mangle]
pub unsafe extern fn abort() -> ! {
    loop {}
}

#[cfg(not(test))]
#[no_mangle]
pub unsafe extern fn __aeabi_memcpy4(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    rlibc::memcpy(dest, src, n)
}

#[cfg(not(test))]
#[no_mangle]
pub unsafe extern fn __aeabi_memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    rlibc::memcpy(dest, src, n)
}

#[cfg(not(test))]
#[no_mangle]
pub unsafe extern fn __aeabi_memclr(dest: *mut u8, n: usize) {
    rlibc::memset(dest, 0, n);
}

#[cfg(not(test))]
#[no_mangle]
pub unsafe extern fn __aeabi_memclr4(dest: *mut u8, n: usize) {
    rlibc::memset(dest, 0, n);
}

#[cfg(not(test))]
#[no_mangle]
pub unsafe extern fn __aeabi_memmove4(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    rlibc::memmove(dest, src, n)
}

#[cfg(not(test))]
#[no_mangle]
#[allow(non_upper_case_globals)]
pub static mut __errno: i32 = 0;

fn sin(f: f32) -> f32 {
    unsafe { sinf32(f) }
}

fn cos(f: f32) -> f32 {
    unsafe { cosf32(f) }
}

fn round(f: f32) -> f32 {
    unsafe { roundf32(f) }
}
