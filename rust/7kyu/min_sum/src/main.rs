fn main() {}

fn min_sum(xs: &[u64]) -> u64 {
    let mut xs = xs.to_vec();
    xs.sort_unstable();
    (0..xs.len() / 2)
        .map(|i| xs[i] * xs[(xs.len() - i - 1)])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(min_sum(&[5, 4, 2, 3]), 22);
        assert_eq!(min_sum(&[12, 6, 10, 26, 3, 24]), 342);
        assert_eq!(min_sum(&[9, 2, 8, 7, 5, 4, 0, 6]), 74);
    }
}
