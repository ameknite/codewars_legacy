fn main() {
    println!("{}", (b'z'))
}

fn add_letters(letters: Vec<char>) -> char {
    if letters.is_empty() {
        return 'z';
    }
    let value = letters
        .iter()
        .fold(0, |acc, &letter| match acc + letter as u8 - b'a' + 1 {
            1..=26 => acc + letter as u8 - b'a' + 1,
            _ => acc + letter as u8 - b'a' - (b'z' - b'a'),
        });
    char::from(value + 96)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(add_letters(vec!['a', 'b', 'c']), 'f');
        assert_eq!(add_letters(vec!['z']), 'z');
        assert_eq!(add_letters(vec!['a', 'b']), 'c');
        assert_eq!(add_letters(vec!['c']), 'c');
        assert_eq!(add_letters(vec!['z', 'a']), 'a');
        assert_eq!(add_letters(vec!['y', 'c', 'b']), 'd');
        assert_eq!(add_letters(vec![]), 'z');
    }
}
