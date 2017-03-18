#[cfg(feature = "semihosted")]
pub unsafe fn write0(ptr: *const u8) {
    // See newlib for details (libgloss/arm/swi.h)
    asm!(
        "bkpt 0xAB"
        :  // Outputs
        : "{r0}"(0x04), "{r1}"(ptr)  // Inputs
        : "r0", "r1", "r2", "r3", "ip", "lr", "memory", "cc"  // Clobbers
    );
}

#[cfg(not(feature = "semihosted"))]
pub unsafe fn write0(_ptr: *const u8) {
}

macro_rules! debug {
    ($fmt:expr, $($args:tt)*) => ({
        let s = ::collections::fmt::format(format_args!(concat!($fmt, "\n\0"), $($args)*));
        #[allow(unused_unsafe)] unsafe { ::debug::write0(s.as_ptr()); }
    });
    ($msg:expr) => ({
        debug!($msg,);
    });
}
