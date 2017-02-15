#![no_std]
#![feature(lang_items)]

#[no_mangle]
pub extern fn it_works(a: i32, b: i32) -> i32 {
    (a + a) / (b * b * b)
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments,
                               _file: &'static str,
                               _line: u32) -> ! {
    loop {}
}
