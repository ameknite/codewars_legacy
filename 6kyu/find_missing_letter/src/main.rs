fn main() {}

fn find_missing_letter(chars: &[char]) -> char {
    (*chars.first().unwrap()..=*chars.last().unwrap())
        .into_iter()
        .find(|&x| !chars.contains(&x))
        .unwrap()
}

// fn find_missing_letter(chars: &[char]) -> char {
//     (chars.windows(2)                        //Convert into a list of arrays length 2, for ease of viewing past and next character.
//           .map(|w| (w[0] as u8, w[1] as u8)) //Convert the character array to a byte tuple.
//           .find(|&w| w.0 + 1 != w.1)         //Example: 'a' + 1 != 'b'? Found a missing character.
//           .unwrap().0 + 1) as char           //Add 1 to previous character, to get the correct character.
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_tests() {
        assert_eq!(find_missing_letter(&['a', 'b', 'c', 'd', 'f']), 'e');
        assert_eq!(find_missing_letter(&['O', 'Q', 'R', 'S']), 'P');
    }
}
