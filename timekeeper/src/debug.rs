
extern {
    fn puts(str: *const u8) -> i32;
}

pub fn write(message: &str) {
    unsafe { puts(message.as_ptr()); }
}

macro_rules! breakpoint {
    () => (unsafe { asm!("BKPT" :::: "volatile") })
}
