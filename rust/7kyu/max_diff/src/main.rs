fn main() {}

fn max_diff(numbers: &[i32]) -> i32 {
    if numbers.len() < 2 {
        return 0;
    }
    numbers.iter().max().unwrap() - numbers.iter().min().unwrap()
}

// use itertools::Itertools;
// use itertools::MinMaxResult::MinMax;

// fn max_diff(numbers: &[i32]) -> i32 {
//     if let MinMax(min, max) = numbers.iter().minmax() { max - min } else { 0 }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_1() {
        assert_eq!(max_diff(&[0, 1, 2, 3, 4, 5, 6]), 6);
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(max_diff(&[-0, 1, 2, -3, 4, 5, -6]), 11);
    }

    #[test]
    fn test_sample_3() {
        assert_eq!(max_diff(&[0, 1, 2, 3, 4, 5, 16]), 16);
    }

    #[test]
    fn test_sample_4() {
        assert_eq!(max_diff(&[16]), 0);
    }

    #[test]
    fn test_sample_5() {
        assert_eq!(max_diff(&[]), 0);
    }
}
