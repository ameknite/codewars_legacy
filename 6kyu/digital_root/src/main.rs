fn main() {}

fn digital_root(n: i64) -> i64 {
    if n < 10 {
        return n;
    }
    let mut sum = 0;
    let mut n = n;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    digital_root(sum)
}

// fn digital_root(n: i64) -> i64 {
//     (n - 1) % 9 + 1
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(digital_root(16), 7);
        assert_eq!(digital_root(493193), 2);
    }
}
