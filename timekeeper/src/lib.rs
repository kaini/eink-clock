#![no_std]
#![feature(lang_items)]
#![feature(collections)]
#![feature(asm)]

extern crate my_allocator;
extern crate collections;

#[macro_use]
mod debug;

use collections::fmt::format;

#[no_mangle]
pub extern fn rust_main() -> ! {
    panic!("The main function quit!");
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(msg: core::fmt::Arguments,
                               file: &'static str,
                               line: u32) -> ! {
    let s = format(format_args!("{} ({}:{})", msg, file, line));
    debug::write(s.as_str());
    breakpoint!();
    loop { }
}

#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr0() -> ! {
    breakpoint!();
    loop { }
}
