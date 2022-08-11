fn main() {}

fn name_shuffler(s: &str) -> String {
    s.split_whitespace().rev().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(name_shuffler("john McClane"), "McClane john");
        assert_eq!(name_shuffler("Mary jeggins"), "jeggins Mary");
        assert_eq!(name_shuffler("tom jerry"), "jerry tom");
    }
}
