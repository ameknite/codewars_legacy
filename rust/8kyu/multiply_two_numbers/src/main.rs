fn main() {}

fn multiply(a: i64, b: i64) -> i64 {
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_simple() {
        assert_eq!(multiply(1, 2), 2);
        assert_eq!(multiply(-50, 4), -200);
        assert_eq!(multiply(-10, -10), 100);
    }
}
