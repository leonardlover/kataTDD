fn reverse(s: &str) -> String {
    let chars = s.chars();
    let rev_chars = chars.rev();
    let rev_str = rev_chars.collect();
    rev_str
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
}

