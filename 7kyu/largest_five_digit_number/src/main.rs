fn main() {}

use itertools::Itertools;

fn largest_five_digit_number(num: &str) -> u32 {
    (0..=num.len() - 5)
        .map(|x| {
            num.chars()
                .skip(x)
                .take(5)
                .map(|c| c.to_digit(10).unwrap())
                .fold(0, |acc, x| acc * 10 + x)
        })
        .max()
        .unwrap()
}

// fn largest_five_digit_number(num: &str) -> u32 {
//     (0..num.len() - 4).filter_map(|start| num[start..start+5].parse::<u32>().ok()).max().unwrap()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(largest_five_digit_number(&"1234567890"), 67890);
        assert_eq!(largest_five_digit_number(&"731674765"), 74765);
    }
}
