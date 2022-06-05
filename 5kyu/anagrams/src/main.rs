fn main() {}

fn anagrams(word: &str, words: &[String]) -> Vec<String> {
    words
        .iter()
        .filter(|&w| is_anagram(word, w))
        .cloned()
        .collect()
}

fn is_anagram(word: &str, other: &str) -> bool {
    if word.len() != other.len() {
        return false;
    }
    let mut word_chars: Vec<_> = word.chars().collect();
    let mut other_chars: Vec<_> = other.chars().collect();
    word_chars.sort_unstable();
    other_chars.sort_unstable();
    word_chars == other_chars
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        do_test("abba", &["aabb", "abcd", "bbaa", "dada"], &["aabb", "bbaa"]);

        do_test(
            "racer",
            &["crazer", "carer", "racar", "caers", "racer"],
            &["carer", "racer"],
        );
    }

    fn do_test(word: &str, words: &[&str], exp: &[&str]) {
        let words: Vec<String> = words.iter().map(|w| w.to_string()).collect();
        let expected: Vec<String> = exp.iter().map(|w| w.to_string()).collect();
        let got = anagrams(word, &words);
        assert_eq!(
            got, expected,
            "Failed with word: \"{}\"\nwords: {:#?}",
            word, words
        );
    }
}
