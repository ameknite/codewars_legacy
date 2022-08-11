fn main() {}

fn reverse_words(s: &str) -> String {
    let mut spaces = 1;
    s.split(" ")
        .map(|x| {
            {
                x.chars().filter(|&x| {
                    if x == ' ' {
                        spaces += 1;
                    }
                    x != ' '
                })
            }
            .rev()
            .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(&" ".repeat(spaces))
}

// fn reverse_words(str: &str) -> String {
//     str.to_string()
//       .split(" ")
//       .map(|sub| sub.chars().rev().collect())
//       .collect::<Vec<String>>()
//       .join(" ")
// }

#[test]
fn sample_test() {
    assert_eq!(
        reverse_words("The quick brown fox jumps over the lazy dog."),
        "ehT kciuq nworb xof spmuj revo eht yzal .god"
    );
    assert_eq!(reverse_words("apple"), "elppa");
    assert_eq!(reverse_words("a b c d"), "a b c d");
    assert_eq!(
        reverse_words("double  spaced  words"),
        "elbuod  decaps  sdrow"
    );
}
