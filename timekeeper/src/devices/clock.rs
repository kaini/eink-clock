use rawhw::{rtc, syscon, nvic};

pub unsafe fn init() {
    syscon::syscfg::rtcclk::set(syscon::RtcClock::OneHz);
    rtc::cr::rtcstart::set(true);
    wait_for_ready();
    rtc::icsc::rtcic::set(true);
    nvic::iser::set(1 << nvic::RTC);
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

pub fn reset_time() {
    unsafe {
        rtc::lr::set(0);
    }
}

pub fn set_interrupt_time(time: i32) {
    unsafe {
        rtc::mr::set(time as u32);
    }
}

pub unsafe extern fn rtc_interrupt() {
    rtc::icr::rtcicr::set(true);
}
