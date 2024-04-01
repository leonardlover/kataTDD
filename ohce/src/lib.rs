fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

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

