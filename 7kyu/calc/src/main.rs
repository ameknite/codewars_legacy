fn main() {}

fn calc(array: Vec<i32>) -> i32 {
    array
        .iter()
        .zip(1..)
        .map(|(&x, i)| {
            let mut x = x;
            if x > 0 {
                x = x * x;
            };
            if i % 3 == 0 {
                x = x * 3;
            };
            if i % 5 == 0 {
                x = -x;
            };
            x
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(calc(vec![0, 2, 1, -6, -3, 3]), 31);
        assert_eq!(calc(vec![0]), 0);
        assert_eq!(calc(vec![1, 1, 1, 1, 1]), 5);
        assert_eq!(calc(vec![1, 1, -9, 9, 16, -15, -45, -73, 26]), 1665);
        assert_eq!(calc(vec![1, -1, 10, -9, 16, 15, 45, -73, -26]), 2584);
        assert_eq!(calc(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 0);
        assert_eq!(calc(vec![-5, -5, -5, -5, -5, -5, -5]), -45);
    }
}
