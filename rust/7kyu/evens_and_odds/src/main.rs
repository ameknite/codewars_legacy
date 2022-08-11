fn main() {}

fn evens_and_odds(n: u64) -> String {
    match n % 2 {
        0 => format!("{n:b}"),
        _ => format!("{n:x}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(evens_and_odds(2), "10");
        assert_eq!(evens_and_odds(13), "d");
    }
}
