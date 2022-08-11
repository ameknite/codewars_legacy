fn main() {}

// fn switcher(numbers: Vec<&str>) -> String {
//     numbers
//         .iter()
//         .map(|&s| match s {
//             "29" => ' ',
//             "28" => '?',
//             "27" => '!',
//             "26" => 'a',
//             "25" => 'b',
//             "24" => 'c',
//             "23" => 'd',
//             "22" => 'e',
//             "21" => 'f',
//             "20" => 'g',
//             "19" => 'h',
//             "18" => 'i',
//             "17" => 'j',
//             "16" => 'k',
//             "15" => 'l',
//             "14" => 'm',
//             "13" => 'n',
//             "12" => 'o',
//             "11" => 'p',
//             "10" => 'q',
//             "9" => 'r',
//             "8" => 's',
//             "7" => 't',
//             "6" => 'u',
//             "5" => 'v',
//             "4" => 'w',
//             "3" => 'x',
//             "2" => 'y',
//             "1" => 'z',
//             _ => panic!(),
//         })
//         .collect()
// }

fn switcher(numbers: Vec<&str>) -> String {
    numbers
        .iter()
        .flat_map(|s| s.parse())
        .map(|x| match x {
            29 => ' ',
            28 => '?',
            27 => '!',
            n => (b'z' + 1 - n).into(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::switcher;

    #[test]
    fn example_tests() {
        assert_eq!(
            switcher(vec!["24", "12", "23", "22", "4", "26", "9", "8"]),
            "codewars"
        );
        assert_eq!(
            switcher(vec![
                "25", "7", "8", "4", "14", "23", "8", "25", "23", "29", "16", "16", "4"
            ]),
            "btswmdsbd kkw"
        );
        assert_eq!(switcher(vec!["4", "24"]), "wc");
    }
}
