fn main() {}

use std::collections::HashMap;

fn count_duplicates(text: &str) -> u32 {
    let mut counts = HashMap::new();
    text.to_lowercase().chars().for_each(|x| {
        *counts.entry(x).or_insert(0) += 1;
    });
    counts.values().filter(|&&x| x > 1).count() as u32
}

// use itertools::Itertools;

// fn count_duplicates(text: &str) -> u32 {
//     text.to_lowercase().chars().counts().values().filter(|&&i| i > 1).count() as u32
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abcde() {
        assert_eq!(count_duplicates("abcde"), 0);
    }

    #[test]
    fn test_abcdea() {
        assert_eq!(count_duplicates("abcdea"), 1);
    }

    #[test]
    fn test_indivisibility() {
        assert_eq!(count_duplicates("indivisibility"), 1);
    }
}
