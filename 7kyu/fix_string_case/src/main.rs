fn main() {}

use std::cmp::Ordering;

fn solve(s: &str) -> String {
    match s
        .chars()
        .map(|c| match c.is_uppercase() {
            true => 1,
            false => -1,
        })
        .sum::<i32>()
        .cmp(&0)
    {
        Ordering::Greater => s.to_uppercase(),
        _ => s.to_lowercase(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(solve("code"), "code");
        assert_eq!(solve("CODe"), "CODE");
        assert_eq!(solve("COde"), "code");
        assert_eq!(solve("Code"), "code");
    }
}
