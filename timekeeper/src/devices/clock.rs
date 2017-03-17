use rawhw::{rtc, syscon};

pub struct Clock {
    _phantom: ()
}

impl Clock {
    pub unsafe fn new() -> Clock {
        syscon::syscfg::rtcclk::set(syscon::RtcClock::ONE_KHZ);
        rtc::cr::rtcstart::set(true);

        let clock = Clock { _phantom: () };
        clock.wait_for_ready();
        clock
    }

    fn wait_for_ready(&self) {
        // Wait for at least three different values
        let a = self.current_time();
        let mut b = self.current_time();
        while b == a {
            b = self.current_time();
        }
        let mut c = self.current_time();
        while c == a || c == b {
            c = self.current_time();
        }
    }

    pub fn current_time(&self) -> u32 {
        unsafe {
            rtc::dr::get()
        }
    }
}
