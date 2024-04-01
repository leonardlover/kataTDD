mod day_moment;

#[cfg(test)]
mod mocks;

#[cfg(test)]
use mocks::Local;

use day_moment::{DayMoment, get_day_moment};
use unicode_segmentation::UnicodeSegmentation;

fn greeting(name: &str) -> String {
    let day_moment = get_day_moment();
    match day_moment {
        DayMoment::Morning => format!("¡Buenos días {}!", name),
        DayMoment::Afternoon => format!("¡Buenas tardes {}!", name),
        DayMoment::Evening => format!("¡Buenas noches {}!", name),
    }
}

fn reverse(s: &str) -> String {
    s.graphemes(true).rev().collect()
}

#[cfg(test)]
mod tests {
    use chrono::NaiveTime;
    use mocks::set_timestamp;
    use super::*;

    #[test]
    fn greeting_morning_lower_bound() {
        // time = 6:00
        set_timestamp(1034154000);
        assert_eq!(
            NaiveTime::parse_from_str("06:00:00", "%H:%M:%S"),
            Ok(Local::now().time())
        );
        assert_eq!("¡Buenos días Leonardo!", greeting("Leonardo"));
    }

    #[test]
    fn greeting_morning() {
        // time = 9:30
        set_timestamp(1034166600);
        assert_eq!(
            NaiveTime::parse_from_str("09:30:00", "%H:%M:%S"),
            Ok(Local::now().time())
        );
        assert_eq!("¡Buenos días Leonardo!", greeting("Leonardo"));
    }

    #[test]
    fn greeting_morning_upper_bound() {
        // time = 11:59
        set_timestamp(1034175540);
        assert_eq!(
            NaiveTime::parse_from_str("11:59:00", "%H:%M:%S"),
            Ok(Local::now().time())
        );
        assert_eq!("¡Buenos días Leonardo!", greeting("Leonardo"));
    }

    #[test]
    fn greeting_afternoon_lower_bound() {
        // time = 12:00
        set_timestamp(1034175600);
        assert_eq!(
            NaiveTime::parse_from_str("12:00:00", "%H:%M:%S"),
            Ok(Local::now().time())
        );
        assert_eq!("¡Buenas tardes Tomás!", greeting("Tomás"));
    }

    #[test]
    fn greeting_afternoon() {
        // time = 18:15
        set_timestamp(1034198100);
        assert_eq!(
            NaiveTime::parse_from_str("18:15:00", "%H:%M:%S"),
            Ok(Local::now().time())
        );
        assert_eq!("¡Buenas tardes Tomás!", greeting("Tomás"));
    }

    #[test]
    fn greeting_afternoon_upper_bound() {
        // time = 19:59
        set_timestamp(1034204340);
        assert_eq!(
            NaiveTime::parse_from_str("19:59:00", "%H:%M:%S"),
            Ok(Local::now().time())
        );
        assert_eq!("¡Buenas tardes Tomás!", greeting("Tomás"));
    }

    #[test]
    fn greeting_evening_lower_bound() {
        // time = 20:00
        set_timestamp(1034204400);
        assert_eq!(
            NaiveTime::parse_from_str("20:00:00", "%H:%M:%S"),
            Ok(Local::now().time())
        );
        assert_eq!("¡Buenas noches Paula!", greeting("Paula"));
    }

    #[test]
    fn greeting_evening() {
        // time = 01:45
        set_timestamp(1034138700);
        assert_eq!(
            NaiveTime::parse_from_str("01:45:00", "%H:%M:%S"),
            Ok(Local::now().time())
        );
        assert_eq!("¡Buenas noches Paula!", greeting("Paula"));
    }

    #[test]
    fn greeting_evening_upper_bound() {
        // time = 05:59
        set_timestamp(1034153940);
        assert_eq!(
            NaiveTime::parse_from_str("05:59:00", "%H:%M:%S"),
            Ok(Local::now().time())
        );
        assert_eq!("¡Buenas noches Paula!", greeting("Paula"));
    }

    #[test]
    fn reverse_ascii_single_char() {
        assert_eq!("a", reverse("a"));
    }

    #[test]
    fn reverse_ascii_two_char() {
        assert_eq!("ym", reverse("my"));
    }

    #[test]
    fn reverse_ascii_multi_char() {
        assert_eq!("yratnemele", reverse("elementary"));
    }

    #[test]
    fn reverse_utf8_single_grapheme() {
        assert_eq!("á", reverse("á"));
    }

    #[test]
    fn reverse_utf8_two_graphemes() {
        assert_eq!("üá", reverse("áü"));
    }

    #[test]
    fn reverse_utf8_multi_grapheme() {
        assert_eq!("regnidörhcS", reverse("Schrödinger"));
    }

    #[test]
    fn reverse_utf8_multi_grapheme_swedish() {
        assert_eq!("mörtsÅ", reverse("Åström"));
    }

    #[test]
    fn reverse_utf8_multi_grapheme_emoji() {
        assert_eq!("❌✅", reverse("✅❌"));
    }
}

