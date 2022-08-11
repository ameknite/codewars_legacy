fn main() {}

fn is_prime(x: i64) -> bool {
    if x < 2 {
        return false;
    }
    let limit = (x as f64).sqrt() as i64;
    for i in 2..=limit {
        if x % i == 0 {
            return false;
        }
    }
    true
}

// fn is_prime(x: i64) -> bool {
//     if x == 2 {
//         return true;
//     };
//     if x < 2 || x % 2 == 0 {
//         return false;
//     };
//     (3..)
//         .step_by(2)
//         .take_while(|i| i * i <= x)
//         .all(|i| x % i != 0)
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert!(!is_prime(0), "0 is not prime");
        assert!(!is_prime(1), "1 is not prime");
        assert!(is_prime(2), "2 is prime");
        assert!(is_prime(73), "73 is prime");
        assert!(!is_prime(75), "75 is not prime");
        assert!(!is_prime(-1), "-1 is not prime");
    }

    #[test]
    fn prime_tests() {
        assert!(is_prime(3), "3 is prime");
        assert!(is_prime(5), "5 is prime");
        assert!(is_prime(7), "7 is prime");
        assert!(is_prime(41), "41 is prime");
        assert!(is_prime(5099), "5099 is prime");
    }

    #[test]
    fn not_prime_tests() {
        assert!(!is_prime(4), "4 is not prime");
        assert!(!is_prime(6), "6 is not prime");
        assert!(!is_prime(8), "8 is not prime");
        assert!(!is_prime(9), "9 is not prime");
        assert!(!is_prime(45), "45 is not prime");
        assert!(!is_prime(-5), "-5 is not prime");
        assert!(!is_prime(-8), "-8 is not prime");
        assert!(!is_prime(-41), "-41 is not prime");
    }
}
