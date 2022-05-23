fn main() {}

fn high_and_low(numbers: &str) -> String {
    let max = numbers
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .max()
        .unwrap();
    let min = numbers
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .min()
        .unwrap();
    format!("{max} {min}")
}

// extern crate itertools;
// use itertools::Itertools;

// fn high_and_low(numbers: &str) -> String {
//     let (min, max): (i32, i32) = numbers
//         .split_whitespace()
//         .map(|s| s.parse().unwrap())
//         .minmax()
//         .into_option()
//         .unwrap();
//     format!("{} {}", max, min)
// }


#[test]
fn example_test_1() {
    assert_eq!("42 -9", high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"));
}

#[test]
fn example_test_2() {
    assert_eq!("3 1", high_and_low("1 2 3"));
}
