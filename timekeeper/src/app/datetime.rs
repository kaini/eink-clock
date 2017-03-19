use devices::dcf77::PAYLOAD_BITS;
use core::result::Result;

pub fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

pub fn day_count(year: i32, month: i32) -> i32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => { 31 }
        4 | 6 | 9 | 11 => { 30 }
        2 => { if is_leap_year(year) { 29 } else { 28 } }
        _ => unreachable!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Datetime {
    // Time in UTC+timezone
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
    second: i32,

    // Timezone in seconds
    timezone: i32,
}

fn parity(data: &[bool]) -> bool {
    let mut result = false;
    for &bit in data {
        result ^= bit;
    }
    result
}

fn bcd_digit(data: &[bool]) -> i32  {
    let mut result = 0;
    let mut write_bit = 1;
    for &bit in data {
        if bit {
            result |= write_bit;
        }
        write_bit <<= 1;
    }
    result
}

fn floor_div(a: i32, b: i32) -> i32 {
    if a < 0 && a % b != 0 {
        a / b - 1
    } else {
        a / b
    }
}

fn modulo(a: i32, b: i32) -> i32 {
    // See http://stackoverflow.com/a/41422009
    (a % b + b) % b
}

impl Datetime {
    pub fn from_dcf77(data: &[bool]) -> Result<Datetime, &'static str> {
        if !data[19] {
            return Err("Bit 20 is not 1");
        }
        if data[27] != parity(&data[20..27]) {
            return Err("Minute parity is wrong");
        }
        if data[34] != parity(&data[28..34]) {
            return Err("Hour parity is wrong");
        }
        if data[57] != parity(&data[35..57]) {
            return Err("Date parity is wrong");
        }

        let year = 2000 + bcd_digit(&data[53..57]) * 10 + bcd_digit(&data[49..53]);
        let month = bcd_digit(&data[48..49]) * 10 + bcd_digit(&data[44..48]);
        let day = bcd_digit(&data[39..41]) * 10 + bcd_digit(&data[35..39]);
        let hour = bcd_digit(&data[32..34]) * 10 + bcd_digit(&data[28..32]);
        let minute = bcd_digit(&data[24..27]) * 10 + bcd_digit(&data[20..24]);
        let timezone = ((data[16] as i32 * 2) + (data[17] as i32)) * 60 * 60;

        Datetime::new(year, month, day, hour, minute, 0, timezone)
    }

    pub fn new(year: i32, month: i32, day: i32, hour: i32, minute: i32, second: i32, timezone: i32) -> Result<Datetime, &'static str> {
        if !(2000 <= year) {
            return Err("Year not in range");
        }
        if !(1 <= month && month <= 12) {
            return Err("Month not in range");
        }
        if !(1 <= day && day <= day_count(year, month)) {
            return Err("Day not in range");
        }
        if !(0 <= hour && hour <= 23) {
            return Err("Hour not in range");
        }
        if !(0 <= minute && minute <= 59) {
            return Err("Minute not in range");
        }
        if !(0 <= second && second <= 59) {
            return Err("Second not in range");
        }

        Ok(Datetime {
            year: year,
            month: month,
            day: day,
            hour: hour,
            minute: minute,
            second: second,
            timezone: timezone,
        })
    }

    pub fn year(&self) -> i32 { self.year }
    pub fn month(&self) -> i32 { self.month }
    pub fn day(&self) -> i32 { self.day }
    pub fn hour(&self) -> i32 { self.hour }
    pub fn minute(&self) -> i32 { self.minute }
    pub fn second(&self) -> i32 { self.second }
    pub fn timezone(&self) -> i32 { self.timezone }

    /// Switches the timezone to the target timezone in seconds.
    pub fn switch_timezone(&mut self, target_timezone: i32) {
        let delta = target_timezone - self.timezone;
        self.offset_seconds(delta);
        self.timezone = target_timezone;
    }

    pub fn offset_seconds(&mut self, offset: i32) {
        self.second += offset;
        self.fix_second_overflow();
    }

    pub fn offset_minutes(&mut self, offset: i32) {
        self.minute += offset;
        self.fix_minute_overflow();
    }

    pub fn offset_hours(&mut self, offset: i32) {
        self.hour += offset;
        self.fix_hour_overflow();
    }

    pub fn offset_days(&mut self, offset: i32) {
        self.day += offset;
        self.fix_day_overflow();
    }

    pub fn is_before(a: &Datetime, b: &Datetime) -> bool {
        let mut aa = a.clone();
        aa.switch_timezone(b.timezone());
        (
            (aa.year, aa.month, aa.day, aa.hour, aa.minute, aa.second) <
            (b.year, b.month, b.day, b.hour, b.minute, b.second)
        )
    }

    pub fn is_after(a: &Datetime, b: &Datetime) -> bool {
        let mut aa = a.clone();
        aa.switch_timezone(b.timezone());
        (
            (aa.year, aa.month, aa.day, aa.hour, aa.minute, aa.second) >
            (b.year, b.month, b.day, b.hour, b.minute, b.second)
        )
    }

    pub fn is_same_timepoint(a: &Datetime, b: &Datetime) -> bool {
        let mut aa = a.clone();
        aa.switch_timezone(b.timezone());
        (
            (aa.year, aa.month, aa.day, aa.hour, aa.minute, aa.second) ==
            (b.year, b.month, b.day, b.hour, b.minute, b.second)
        )
    }

    /// Returns the weekday with 0 = Sunday, 1 = Monday, ..., 6 = Saturday.
    pub fn weekday(&self) -> i32 {
        // See https://en.wikipedia.org/wiki/Determination_of_the_day_of_the_week#Implementation-dependent_methods
        const T: [i32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
        let y = self.year - (self.month < 3) as i32;
        (y + y / 4 - y / 100 + y / 400 + T[self.month as usize - 1] + self.day) % 7
    }

    fn fix_second_overflow(&mut self) {
        self.minute += floor_div(self.second, 60);
        self.second = modulo(self.second, 60);
        self.fix_minute_overflow();
    }

    fn fix_minute_overflow(&mut self) {
        self.hour += floor_div(self.minute, 60);
        self.minute = modulo(self.minute, 60);
        self.fix_hour_overflow();
    }

    fn fix_hour_overflow(&mut self) {
        self.day += floor_div(self.hour, 24);
        self.hour = modulo(self.hour, 24);
        self.fix_day_overflow();
    }

    fn fix_day_overflow(&mut self) {
        while self.day > day_count(self.year, self.month) {
            self.day -= day_count(self.year, self.month);
            self.month += 1;
            self.fix_month_overflow();
        }
        while self.day < 1 {
            self.day += if self.month == 1 {
                day_count(self.year - 1, 12)
            } else {
                day_count(self.year, self.month - 1)
            };
            self.month -= 1;
            self.fix_month_overflow();
        }
    }

    fn fix_month_overflow(&mut self) {
        self.year += floor_div(self.month - 1, 12);
        self.month = modulo(self.month - 1, 12) + 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parity() {
        assert_eq!(parity(&[true, true, true, false]), true);
        assert_eq!(parity(&[true, false, false, false, false]), true);
        assert_eq!(parity(&[false, false, false]), false);
        assert_eq!(parity(&[true, false, true, false]), false);
    }

    #[test]
    fn test_bcd_digit() {
        assert_eq!(bcd_digit(&[true, false, true]), 5);
        assert_eq!(bcd_digit(&[false, true, false, true]), 10);
        assert_eq!(bcd_digit(&[true, false, true, false]), 5);
    }

    #[test]
    fn test_floor_div() {
        assert_eq!(floor_div(20, 10), 2);
        assert_eq!(floor_div(11, 10), 1);
        assert_eq!(floor_div(1, 10), 0);
        assert_eq!(floor_div(0, 10), 0);
        assert_eq!(floor_div(-1, 10), -1);
        assert_eq!(floor_div(-11, 10), -2);
        assert_eq!(floor_div(-20, 10), -2);
    }

    #[test]
    fn test_modulo() {
        assert_eq!(modulo(20, 10), 0);
        assert_eq!(modulo(11, 10), 1);
        assert_eq!(modulo(1, 10), 1);
        assert_eq!(modulo(0, 10), 0);
        assert_eq!(modulo(-1, 10), 9);
        assert_eq!(modulo(-11, 10), 9);
        assert_eq!(modulo(-20, 10), 0);
    }

    #[test]
    fn test_datetime_new_success1() {
        // Do, 16.02.17 04:28:00, WZ
        // http://www.dcf77logs.de/ViewLog.aspx?mode=dcf77&file=DCFLog03290.log
        let s = "0011111000111000010100010100001000101101000101000111010001";
        let mut sig = [false; PAYLOAD_BITS];
        for (i, c) in s.char_indices() {
            sig[i] = c == '1';
        }
        let result = Datetime::from_dcf77(&sig).unwrap();
        assert_eq!(result.hour(), 4);
        assert_eq!(result.minute(), 28);
        assert_eq!(result.second(), 0);
        assert_eq!(result.timezone(), 3600);
        assert_eq!(result.day(), 16);
        assert_eq!(result.month(), 2);
        assert_eq!(result.year(), 2017);
    }

    #[test]
    fn test_datetime_new_success2() {
        // Do, 10.11.16 13:15:00, WZ
        // http://www.dcf77logs.de/ViewLog.aspx?mode=dcf77&file=DCFLog03189.log
        let s = "0010111011010000010110101001110010100001000110001011010001";
        let mut sig = [false; PAYLOAD_BITS];
        for (i, c) in s.char_indices() {
            sig[i] = c == '1';
        }
        let result = Datetime::from_dcf77(&sig).unwrap();
        assert_eq!(result.hour(), 13);
        assert_eq!(result.minute(), 15);
        assert_eq!(result.second(), 0);
        assert_eq!(result.timezone(), 3600);
        assert_eq!(result.day(), 10);
        assert_eq!(result.month(), 11);
        assert_eq!(result.year(), 2016);
    }

    #[test]
    fn test_datetime_new_error() {
        let s = "0010111011010000010111101001110010100001000110001011010001";
        let mut sig = [false; PAYLOAD_BITS];
        for (i, c) in s.char_indices() {
            sig[i] = c == '1';
        }
        let result = Datetime::from_dcf77(&sig);
        assert!(result.is_err());
    }

    #[test]
    fn test_offset_days() {
        let mut t = Datetime::new(2017, 2, 19, 1, 2, 3, 3600).unwrap();
        t.offset_days(1);
        assert_eq!(t, Datetime::new(2017, 2, 20, 1, 2, 3, 3600).unwrap());
        t.offset_days(-2);
        assert_eq!(t, Datetime::new(2017, 2, 18, 1, 2, 3, 3600).unwrap());
    }

    #[test]
    fn test_offset_days_month_switch() {
        let mut t = Datetime::new(2012, 2, 19, 1, 2, 3, 3600).unwrap();
        t.offset_days(10);
        assert_eq!(t, Datetime::new(2012, 2, 29, 1, 2, 3, 3600).unwrap());
        t.offset_days(5);
        assert_eq!(t, Datetime::new(2012, 3, 5, 1, 2, 3, 3600).unwrap());
        t.offset_days(-6);
        assert_eq!(t, Datetime::new(2012, 2, 28, 1, 2, 3, 3600).unwrap());
        t.offset_days(-10);
        assert_eq!(t, Datetime::new(2012, 2, 18, 1, 2, 3, 3600).unwrap());
    }

    #[test]
    fn test_offset_days_year_switch() {
        let mut t = Datetime::new(2012, 12, 30, 1, 2, 3, 3600).unwrap();
        t.offset_days(2);
        assert_eq!(t, Datetime::new(2013, 1, 1, 1, 2, 3, 3600).unwrap());
        t.offset_days(-3);
        assert_eq!(t, Datetime::new(2012, 12, 29, 1, 2, 3, 3600).unwrap());
    }

    #[test]
    fn test_offset_days_big() {
        let mut t = Datetime::new(2012, 11, 30, 1, 2, 3, 3600).unwrap();
        t.offset_days(100);
        assert_eq!(t, Datetime::new(2013, 3, 10, 1, 2, 3, 3600).unwrap());
        t.offset_days(-2103);
        assert_eq!(t, Datetime::new(2007, 6, 7, 1, 2, 3, 3600).unwrap());
    }

    #[test]
    fn test_offset_hours() {
        let mut t = Datetime::new(2012, 12, 30, 1, 2, 3, 3600).unwrap();
        t.offset_hours(54);
        assert_eq!(t, Datetime::new(2013, 1, 1, 7, 2, 3, 3600).unwrap());
        t.offset_hours(-64);
        assert_eq!(t, Datetime::new(2012, 12, 29, 15, 2, 3, 3600).unwrap());
    }

    #[test]
    fn test_offset_minutes() {
        let mut t = Datetime::new(2012, 12, 30, 1, 2, 3, 3600).unwrap();
        t.offset_minutes(54 * 60 + 23);
        assert_eq!(t, Datetime::new(2013, 1, 1, 7, 25, 3, 3600).unwrap());
        t.offset_minutes(-64 * 60 - 25);
        assert_eq!(t, Datetime::new(2012, 12, 29, 15, 0, 3, 3600).unwrap());
    }

    #[test]
    fn test_offset_seconds() {
        let mut t = Datetime::new(2012, 12, 30, 1, 2, 3, 3600).unwrap();
        t.offset_seconds(54 * 60 * 60 + 23 * 60 + 27);
        assert_eq!(t, Datetime::new(2013, 1, 1, 7, 25, 30, 3600).unwrap());
        t.offset_seconds(-64 * 60 * 60 - 25 * 60 - 31);
        assert_eq!(t, Datetime::new(2012, 12, 29, 14, 59, 59, 3600).unwrap());
    }

    #[test]
    fn test_switch_timezone() {
        let mut t = Datetime::new(2012, 12, 30, 1, 2, 3, 3600).unwrap();
        t.switch_timezone(0);
        assert_eq!(t, Datetime::new(2012, 12, 30, 0, 2, 3, 0).unwrap());
        t.switch_timezone(3600 * 2);
        assert_eq!(t, Datetime::new(2012, 12, 30, 2, 2, 3, 3600 * 2).unwrap());
    }

    #[test]
    fn test_is_leap_year() {
        assert_eq!(is_leap_year(2000), true);
        assert_eq!(is_leap_year(1999), false);
        assert_eq!(is_leap_year(2004), true);
        assert_eq!(is_leap_year(1900), false);
    }

    #[test]
    fn test_day_count_february() {
        assert_eq!(day_count(2000, 2), 29);
        assert_eq!(day_count(2001, 2), 28);
    }

    #[test]
    fn test_is_same_timepoint() {
        let a = &Datetime::new(2012, 12, 30, 0, 2, 3, 3600).unwrap();
        let b = &Datetime::new(2012, 12, 29, 22, 2, 3, -3600).unwrap();
        let c = &Datetime::new(2012, 12, 29, 22, 2, 3, 0).unwrap();
        assert_eq!(Datetime::is_same_timepoint(a, b), true);
        assert_eq!(Datetime::is_same_timepoint(b, a), true);
        assert_eq!(Datetime::is_same_timepoint(a, c), false);
        assert_eq!(Datetime::is_same_timepoint(c, a), false);
        assert_eq!(Datetime::is_same_timepoint(b, c), false);
        assert_eq!(Datetime::is_same_timepoint(c, b), false);
    }

    #[test]
    fn test_is_before() {
        let a = &Datetime::new(2012, 12, 30, 0, 2, 3, 3600).unwrap();
        let b = &Datetime::new(2012, 12, 29, 22, 2, 3, -3600).unwrap();
        let c = &Datetime::new(2012, 12, 29, 22, 2, 3, 0).unwrap();
        assert_eq!(Datetime::is_before(a, b), false);
        assert_eq!(Datetime::is_before(b, a), false);
        assert_eq!(Datetime::is_before(a, c), false);
        assert_eq!(Datetime::is_before(c, a), true);
        assert_eq!(Datetime::is_before(b, c), false);
        assert_eq!(Datetime::is_before(c, b), true);
    }

    #[test]
    fn test_is_after() {
        let a = &Datetime::new(2012, 12, 30, 0, 2, 3, 3600).unwrap();
        let b = &Datetime::new(2012, 12, 29, 22, 2, 3, -3600).unwrap();
        let c = &Datetime::new(2012, 12, 29, 22, 2, 3, 0).unwrap();
        assert_eq!(Datetime::is_after(a, b), false);
        assert_eq!(Datetime::is_after(b, a), false);
        assert_eq!(Datetime::is_after(a, c), true);
        assert_eq!(Datetime::is_after(c, a), false);
        assert_eq!(Datetime::is_after(b, c), true);
        assert_eq!(Datetime::is_after(c, b), false);
    }

    #[test]
    fn test_weekday() {
        assert_eq!(Datetime::new(2012, 12, 30, 1, 2, 3, 3600).unwrap().weekday(), 0);
        assert_eq!(Datetime::new(2013, 1, 1, 7, 25, 30, 3600).unwrap().weekday(), 2);
        assert_eq!(Datetime::new(2012, 12, 29, 14, 59, 59, 3600).unwrap().weekday(), 6);
    }
}
