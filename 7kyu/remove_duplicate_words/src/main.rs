fn main() {}

fn remove_duplicate_words(s: &str) -> String {
    let mut v = Vec::new();
    s.split_whitespace().for_each(|w| {
        if !v.contains(&w) {
            v.push(w);
        }
    });
    v.join(" ")
}


// use std::collections::HashSet;

// fn remove_duplicate_words(s: &str) -> String {
//     let mut words: HashSet<&str> = HashSet::new();

//     s.split_whitespace()
//         .filter(|w| words.insert(w))
//         .collect::<Vec<&str>>()
//         .join(" ")
// }

// extern crate itertools;
// use itertools::Itertools;
// use std::collections::HashSet;

// fn remove_duplicate_words(s: &str) -> String {
//     s.split_whitespace().unique().join(" ")
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test_cases() {
        assert_eq!(
            remove_duplicate_words(
                "alpha beta beta gamma gamma gamma delta alpha beta beta gamma gamma gamma delta"
            ),
            "alpha beta gamma delta"
        );
        assert_eq!(
            remove_duplicate_words("my cat is my cat fat"),
            "my cat is fat"
        );
    }
}
