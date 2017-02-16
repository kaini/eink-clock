use devices::dcf77::PAYLOAD_BITS;
use core::result::Result;

#[derive(Debug)]
pub struct Basetime {
    pub hour: i32,
    pub minute: i32,
    pub timezone: i32,
    pub day: i32,
    pub month: i32,
    pub year: i32,
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

impl Basetime {
    pub fn new(data: &[bool; PAYLOAD_BITS]) -> Result<Basetime, &str> {
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

        Ok(Basetime {
            hour: bcd_digit(&data[32..34]) * 10 + bcd_digit(&data[28..32]),
            minute: bcd_digit(&data[24..27]) * 10 + bcd_digit(&data[20..24]),
            timezone: match (data[16], data[17]) {
                (true, false) => { 2 },
                (false, true) => { 1 },
                _             => { return Err("Unknown timezone"); },
            },
            day: bcd_digit(&data[39..41]) * 10 + bcd_digit(&data[35..39]),
            month: bcd_digit(&data[48..49]) * 10 + bcd_digit(&data[44..48]),
            year: 2000 + bcd_digit(&data[53..57]) * 10 + bcd_digit(&data[49..53]),
        })
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
    fn test_basetime_new_success1() {
        // Do, 16.02.17 04:28:00, WZ
        // http://www.dcf77logs.de/ViewLog.aspx?mode=dcf77&file=DCFLog03290.log
        let s = "0011111000111000010100010100001000101101000101000111010001";
        let mut sig = [false; PAYLOAD_BITS];
        for (i, c) in s.char_indices() {
            sig[i] = c == '1';
        }
        let result = Basetime::new(&sig).unwrap();
        assert_eq!(result.hour, 4);
        assert_eq!(result.minute, 28);
        assert_eq!(result.timezone, 1);
        assert_eq!(result.day, 16);
        assert_eq!(result.month, 2);
        assert_eq!(result.year, 2017);
    }

    #[test]
    fn test_basetime_new_success2() {
        // Do, 10.11.16 13:15:00, WZ
        // http://www.dcf77logs.de/ViewLog.aspx?mode=dcf77&file=DCFLog03189.log
        let s = "0010111011010000010110101001110010100001000110001011010001";
        let mut sig = [false; PAYLOAD_BITS];
        for (i, c) in s.char_indices() {
            sig[i] = c == '1';
        }
        let result = Basetime::new(&sig).unwrap();
        assert_eq!(result.hour, 13);
        assert_eq!(result.minute, 15);
        assert_eq!(result.timezone, 1);
        assert_eq!(result.day, 10);
        assert_eq!(result.month, 11);
        assert_eq!(result.year, 2016);
    }

    #[test]
    fn test_basetime_new_error() {
        let s = "0010111011010000010111101001110010100001000110001011010001";
        let mut sig = [false; PAYLOAD_BITS];
        for (i, c) in s.char_indices() {
            sig[i] = c == '1';
        }
        let result = Basetime::new(&sig);
        assert!(result.is_err());
    }
}
