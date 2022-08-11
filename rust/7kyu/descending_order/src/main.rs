fn main() {
    descending_order(1234);
}

fn descending_order(x: u64) -> u64 {
    let mut x = x;
    let mut vector = vec![];
    while x != 0 {
        vector.push(x % 10);
        x /= 10;
    }
    vector.sort_by(|a, b| b.cmp(a));
    vector.iter().fold(0, |acc, elem| acc * 10 + elem)
}

// extern crate itertools;
// use itertools::Itertools;

// fn descending_order(x: u64) -> u64 {
//     x.to_string()
//         .chars()
//         .sorted()
//         .rev()
//         .collect::<String>()
//         .parse::<u64>()
//         .unwrap()
// }

#[test]
fn returns_expected() {
    assert_eq!(descending_order(0), 0);
    assert_eq!(descending_order(1), 1);
    assert_eq!(descending_order(15), 51);
    assert_eq!(descending_order(1021), 2110);
    assert_eq!(descending_order(123456789), 987654321);
    assert_eq!(descending_order(145263), 654321);
    assert_eq!(descending_order(1254859723), 9875543221);
}
