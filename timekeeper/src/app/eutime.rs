use app::datetime::Datetime;

pub fn central_localtime(t: &Datetime) -> Datetime {
    let begin_dst = &{
        let mut begin_dst = Datetime::new(t.year(), 3, 31, 1, 0, 0, 0).unwrap();
        let offset = -begin_dst.weekday();
        begin_dst.offset_days(offset);
        begin_dst
    };
    let end_dst = &{
        let mut end_dst = Datetime::new(t.year(), 10, 31, 1, 0, 0, 0).unwrap();
        let offset = -end_dst.weekday();
        end_dst.offset_days(offset);
        end_dst
    };

    let mut result = t.clone();
    if Datetime::is_before(t, begin_dst) || Datetime::is_after(t, end_dst) || Datetime::is_same_timepoint(t, end_dst) {
        result.switch_timezone(3600);
    } else {
        result.switch_timezone(7200);
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cet_to_cest() {
        let mut t = Datetime::new(2017, 3, 26, 1, 58, 0, 3600).unwrap();
        assert_eq!(central_localtime(&t), Datetime::new(2017, 3, 26, 1, 58, 0, 3600).unwrap());
        t.offset_minutes(1);
        assert_eq!(central_localtime(&t), Datetime::new(2017, 3, 26, 1, 59, 0, 3600).unwrap());
        t.offset_minutes(1);
        assert_eq!(central_localtime(&t), Datetime::new(2017, 3, 26, 3, 0, 0, 7200).unwrap());
        t.offset_minutes(1);
        assert_eq!(central_localtime(&t), Datetime::new(2017, 3, 26, 3, 1, 0, 7200).unwrap());
        
        t = Datetime::new(2019, 3, 31, 1, 58, 0, 3600).unwrap();
        assert_eq!(central_localtime(&t), Datetime::new(2019, 3, 31, 1, 58, 0, 3600).unwrap());
        t.offset_minutes(1);
        assert_eq!(central_localtime(&t), Datetime::new(2019, 3, 31, 1, 59, 0, 3600).unwrap());
        t.offset_minutes(1);
        assert_eq!(central_localtime(&t), Datetime::new(2019, 3, 31, 3, 0, 0, 7200).unwrap());
        t.offset_minutes(1);
        assert_eq!(central_localtime(&t), Datetime::new(2019, 3, 31, 3, 1, 0, 7200).unwrap());
    }

    #[test]
    fn test_cest_to_cet() {
        let mut t = Datetime::new(2017, 10, 29, 2, 58, 0, 7200).unwrap();
        assert_eq!(central_localtime(&t), Datetime::new(2017, 10, 29, 2, 58, 0, 7200).unwrap());
        t.offset_minutes(1);
        assert_eq!(central_localtime(&t), Datetime::new(2017, 10, 29, 2, 59, 0, 7200).unwrap());
        t.offset_minutes(1);
        assert_eq!(central_localtime(&t), Datetime::new(2017, 10, 29, 2, 0, 0, 3600).unwrap());
        t.offset_minutes(1);
        assert_eq!(central_localtime(&t), Datetime::new(2017, 10, 29, 2, 1, 0, 3600).unwrap());

        t = Datetime::new(2021, 10, 31, 2, 58, 0, 7200).unwrap();
        assert_eq!(central_localtime(&t), Datetime::new(2021, 10, 31, 2, 58, 0, 7200).unwrap());
        t.offset_minutes(1);
        assert_eq!(central_localtime(&t), Datetime::new(2021, 10, 31, 2, 59, 0, 7200).unwrap());
        t.offset_minutes(1);
        assert_eq!(central_localtime(&t), Datetime::new(2021, 10, 31, 2, 0, 0, 3600).unwrap());
        t.offset_minutes(1);
        assert_eq!(central_localtime(&t), Datetime::new(2021, 10, 31, 2, 1, 0, 3600).unwrap());
    }
}
