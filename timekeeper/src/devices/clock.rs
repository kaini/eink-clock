use rawhw::{rtc, syscon};

pub unsafe fn init() {
    syscon::syscfg::rtcclk::set(syscon::RtcClock::OneKHz);
    rtc::cr::rtcstart::set(true);
    wait_for_ready();
}

fn wait_for_ready() {
    // Wait for at least three different values
    let a = current_time();
    let mut b = current_time();
    while b == a {
        b = current_time();
    }
    let mut c = current_time();
    while c == a || c == b {
        c = current_time();
    }
}

pub fn current_time() -> i32 {
    unsafe {
        rtc::dr::get() as i32
    }
}

pub fn offset_time(offset: i32) {
    unsafe {
        rtc::dr::set((current_time() + offset) as u32);
    }
}
