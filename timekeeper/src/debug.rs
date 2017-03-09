#[cfg(all(debug_assertions, not(test)))]
pub unsafe fn write0(ptr: *const u8) {
    // See newlib for details (libgloss/arm/swi.h)
    asm!(
        "bkpt 0xAB"
        :  // Outputs
        : "{r0}"(0x04), "{r1}"(ptr)  // Inputs
        : "r0", "r1", "r2", "r3", "ip", "lr", "memory", "cc"  // Clobbers
    );
}

#[cfg(any(not(debug_assertions), test))]
pub unsafe fn write0(_ptr: *const u8) {
}

#[cfg(all(debug_assertions, not(test)))]
macro_rules! breakpoint {
    () => (unsafe { asm!("bkpt" :::: "volatile"); });
}

#[cfg(any(not(debug_assertions), test))]
macro_rules! breakpoint {
    () => ();
}

#[cfg(debug_assertions)]
macro_rules! debug {
    ($fmt:expr, $($args:tt)*) => ({
        let s = ::collections::fmt::format(format_args!(concat!($fmt, "\n\0"), $($args)*));
        unsafe { ::debug::write0(s.as_ptr()); }
    });
    ($msg:expr) => ({
        debug!("{:?}", $expr);
    });
}

#[cfg(not(debug_assertions))]
macro_rules! debug {
    ($fmt:expr, $($args:tt)*) => ();
    ($msg:expr) => ();
}

macro_rules! debug_trace {
    () => (debug!("{}:{}", file!(), line!()))
}
