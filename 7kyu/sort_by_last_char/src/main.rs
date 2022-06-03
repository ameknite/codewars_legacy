fn main() {}

fn sort_by_last_char(s: &str) -> Vec<String> {
    let mut s = s
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    s.sort_by(|a, b| a.chars().next_back().cmp(&b.chars().next_back()));
    s
}

#[cfg(test)]
mod tests {
    use super::sort_by_last_char;

    #[test]
    fn sample_tests() {
        assert_eq!(
            sort_by_last_char("man i need a taxi up to ubud"),
            vec!["a", "need", "ubud", "i", "taxi", "man", "to", "up"]
        );
        assert_eq!(
            sort_by_last_char("what time are we climbing up the volcano"),
            vec!["time", "are", "we", "the", "climbing", "volcano", "up", "what"]
        );
        assert_eq!(
            sort_by_last_char("take me to semynak"),
            vec!["take", "me", "semynak", "to"]
        );
        assert_eq!(
            sort_by_last_char("massage yes massage yes massage"),
            vec!["massage", "massage", "massage", "yes", "yes"]
        );
        assert_eq!(
            sort_by_last_char("take bintang and a dance please"),
            vec!["a", "and", "take", "dance", "please", "bintang"]
        );
    }
}
