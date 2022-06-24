fn main() {}

fn solve(strings: &[String]) -> Vec<usize> {
    strings
        .iter()
        .map(|s| {
            let mut count = 0;
            let mut alphabet = 'a'..='z';
            s.chars().for_each(|c| {
                if c.to_lowercase().next() == alphabet.next() {
                    count += 1;
                }
            });
            count
        })
        .collect()
}

// fn solve(strings: &[String]) -> Vec<usize> {
//     strings
//         .iter()
//         .map(|s| {
//             s.to_ascii_lowercase()
//                 .chars()
//                 .zip('a'..='z')
//                 .filter(|(c0, c1)| c0 == c1)
//                 .count()
//         })
//         .collect()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(
            solve(&["abode".to_string(), "ABc".to_string(), "xyzD".to_string()]),
            vec![4, 3, 1]
        );

        assert_eq!(
            solve(&["abide".to_string(), "ABc".to_string(), "xyz".to_string()]),
            vec![4, 3, 0]
        );

        assert_eq!(
            solve(&[
                "IAMDEFANDJKL".to_string(),
                "thedefgh".to_string(),
                "xyzDEFghijabc".to_string()
            ]),
            vec![6, 5, 7]
        );

        assert_eq!(
            solve(&[
                "encode".to_string(),
                "abc".to_string(),
                "xyzD".to_string(),
                "ABmD".to_string()
            ]),
            vec![1, 3, 1, 3]
        );
    }
}
