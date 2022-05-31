fn main() {
    println!("{}", b'a');
}


fn words_to_marks(s: &str) -> u32 {
    s.chars().map(|x| u32::from(x) - 96).sum()
}

// fn words_to_marks(s: &str) -> u32 {
//     s.bytes().map(|x|(x - b'a' + 1) as u32).sum()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(words_to_marks("attitude"), 100);
        assert_eq!(words_to_marks("friends"), 75);
        assert_eq!(words_to_marks("family"), 66);
        assert_eq!(words_to_marks("selfness"), 99);
        assert_eq!(words_to_marks("knowledge"), 96);
    }
}
