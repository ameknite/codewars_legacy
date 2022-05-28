fn main() {}

fn triangular(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    }
    n * (n + 1) / 2
}

#[cfg(test)]
mod tests {
    use super::triangular;

    #[test]
    fn sample_tests() {
        assert_eq!(triangular(2), 3);
        assert_eq!(triangular(4), 10);
        assert_eq!(triangular(9), 45);
        assert_eq!(triangular(-9), 0);
    }
}
