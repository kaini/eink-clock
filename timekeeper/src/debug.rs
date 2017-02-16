extern {
    fn print_string(ptr: *const u8, size: usize);
}

pub fn write(message: &str) {
    unsafe { print_string(message.as_ptr(), message.len()); }
}

pub fn writeln(message: &str) {
    write(message);
    write("\n");
}

macro_rules! breakpoint {
    () => (unsafe { asm!("BKPT" :::: "volatile") })
}

macro_rules! debug {
    ($fmt:expr, $($args:tt)*) => ({
        let s = ::collections::fmt::format(format_args!($fmt, $($args)*));
        ::debug::writeln(s.as_str());
    });
    ($msg:expr) => ({
        ::debug::writeln($msg);
    });
}

macro_rules! debug_nonl {
    ($fmt:expr, $($args:tt)*) => ({
        let s = ::collections::fmt::format(format_args!($fmt, $($args)*));
        ::debug::write(s.as_str());
    });
    ($msg:expr) => ({
        ::debug::writeln($msg);
    });
}

macro_rules! debug_trace {
    () => (debug!("{}:{}", file!(), line!()))
}
