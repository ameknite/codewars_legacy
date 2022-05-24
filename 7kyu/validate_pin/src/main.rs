fn main() {}

fn validate_pin(pin: &str) -> bool {
    match pin.len() {
        4 | 6 => match pin.parse::<u32>() {
            Ok(_) => match pin.starts_with('+') {
                true => false,
                _ => true,
            },
            _ => false,
        },
        _ => false,
    }
}

// fn validate_pin(pin: &str) -> bool {
//     pin.chars().all(|c| c.is_digit(10)) && (pin.len() == 4 || pin.len() == 6)
// }

#[cfg(test)]
mod tests {
    use super::validate_pin;

    #[test]
    fn invalid_length_tests() {
        assert_eq!(validate_pin("1"), false);
        assert_eq!(validate_pin("12"), false);
        assert_eq!(validate_pin("123"), false);
        assert_eq!(validate_pin("12345"), false);
        assert_eq!(validate_pin("1234567"), false);
        assert_eq!(validate_pin("-1234"), false);
        assert_eq!(validate_pin("1.234"), false);
        assert_eq!(validate_pin("-1.234"), false);
        assert_eq!(validate_pin("00000000"), false);
    }

    #[test]
    fn non_digit_chars_tests() {
        assert_eq!(validate_pin("a234"), false);
        assert_eq!(validate_pin(".234"), false);
    }

    #[test]
    fn valid_pin_tests() {
        assert_eq!(validate_pin("1234"), true);
        assert_eq!(validate_pin("0000"), true);
        assert_eq!(validate_pin("1111"), true);
        assert_eq!(validate_pin("123456"), true);
        assert_eq!(validate_pin("098765"), true);
        assert_eq!(validate_pin("000000"), true);
        assert_eq!(validate_pin("123456"), true);
        assert_eq!(validate_pin("090909"), true);
    }
}
