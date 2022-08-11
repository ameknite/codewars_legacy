fn main() {}

use std::collections::HashMap;

struct Arith<'a> {
    value: &'a str,
}

impl Arith<'_> {
    fn add(&self, word: &str) -> &str {
        let mut map = HashMap::new();
        map.entry("zero").or_insert(0);
        map.entry("one").or_insert(1);
        map.entry("two").or_insert(2);
        map.entry("three").or_insert(3);
        map.entry("four").or_insert(4);
        map.entry("five").or_insert(5);
        map.entry("six").or_insert(6);
        map.entry("seven").or_insert(7);
        map.entry("eight").or_insert(8);
        map.entry("nine").or_insert(9);
        map.entry("ten").or_insert(10);
        map.entry("eleven").or_insert(11);
        map.entry("twelve").or_insert(12);
        map.entry("thirteen").or_insert(13);
        map.entry("fourteen").or_insert(14);
        map.entry("fifteen").or_insert(15);
        map.entry("sixteen").or_insert(16);
        map.entry("seventeen").or_insert(17);
        map.entry("eighteen").or_insert(18);
        map.entry("nineteen").or_insert(19);
        map.entry("twenty").or_insert(20);
        let value = map.get(word).unwrap() + map.get(self.value).unwrap();
        map.keys()
            .find(|&&x| map.get(x).unwrap() == &value)
            .unwrap()
    }
}

// const NUMBER_WORDS: [&str; 21] = [
//     "zero",
//     "one",
//     "two",
//     "three",
//     "four",
//     "five",
//     "six",
//     "seven",
//     "eight",
//     "nine",
//     "ten",
//     "eleven",
//     "twelve",
//     "thirteen",
//     "fourteen",
//     "fifteen",
//     "sixteen",
//     "seventeen",
//     "eighteen",
//     "nineteen",
//     "twenty",
// ];

// struct Arith {
//     value: &'static str,
// }

// impl Arith {
//     fn add(&self, s: &str) -> &'static str {
//         let left = NUMBER_WORDS.iter().position(|&x| x == self.value).unwrap();
//         let right = NUMBER_WORDS.iter().position(|&x| x == s).unwrap();
//         &NUMBER_WORDS[left + right]
//     }
// }

#[test]
fn returns_expected() {
    let c = Arith { value: "three" };
    assert_eq!(c.add("seven"), "ten");
    assert_eq!(c.add("eight"), "eleven");
    assert_eq!(c.add("zero"), "three");
}
