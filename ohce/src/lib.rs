use unicode_segmentation::UnicodeSegmentation;

fn greeting(name: &str) -> String {
    format!("Called greeting({})", name)
}

fn reverse(s: &str) -> String {
    s.graphemes(true).rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_morning_lower_bound() {
        // time = 6:00
        assert_eq!("¡Buenos días Leonardo!", greeting("Leonardo"));
    }

    #[test]
    fn greeting_morning() {
        // time = 9:30
        assert_eq!("¡Buenos días Leonardo!", greeting("Leonardo"));
    }

    #[test]
    fn greeting_morning_upper_bound() {
        // time = 11:59
        assert_eq!("¡Buenos días Leonardo!", greeting("Leonardo"));
    }

    #[test]
    fn greeting_afternoon_lower_bound() {
        // time = 12:00
        assert_eq!("¡Buenas tardes Tomás!", greeting("Tomás"));
    }

    #[test]
    fn greeting_afternoon() {
        // time = 18:15
        assert_eq!("¡Buenas tardes Tomás!", greeting("Tomás"));
    }

    #[test]
    fn greeting_afternoon_upper_bound() {
        // time = 19:59
        assert_eq!("¡Buenas tardes Tomás!", greeting("Tomás"));
    }

    #[test]
    fn greeting_evening_lower_bound() {
        // time = 20:00
        assert_eq!("¡Buenas noches Paula!", greeting("Paula"));
    }

    #[test]
    fn greeting_evening() {
        // time = 01:45
        assert_eq!("¡Buenas noches Paula!", greeting("Paula"));
    }

    #[test]
    fn greeting_evening_upper_bound() {
        // time = 05:59
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

