use core::num;

fn main() {}

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(gcd(30, 12), 6);
        assert_eq!(gcd(8, 9), 1);
        assert_eq!(gcd(1, 1), 1);
    }
}
