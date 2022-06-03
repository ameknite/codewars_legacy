fn main() {}

use std::collections::HashMap;

fn duplicate_encode(word: &str) -> String {
    let mut map = HashMap::new();
    let word = word.to_lowercase();
    word.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);
    word.chars()
        .map(|c| match map[&c] > 1 {
            true => ')',
            false => '(',
        })
        .collect()
}

// fn duplicate_encode(word:&str)->String {
//   let lower = String::from(word).to_lowercase();
//   lower.chars().map(|c| if lower.find(c) == lower.rfind(c) { '(' } else { ')' }).collect()
// }

#[test]
fn run_tests() {
    assert_eq!(duplicate_encode("din"), "(((");
    assert_eq!(duplicate_encode("recede"), "()()()");
    assert_eq!(duplicate_encode("Success"), ")())())", "should ignore case");
    assert_eq!(duplicate_encode("(( @"), "))((");
}
