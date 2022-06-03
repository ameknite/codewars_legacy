fn main() {}

fn persistence(num: u64) -> u64 {
    let mut n = num;
    let mut count = 0;
    while n > 9 {
        let mut product = 1;
        while n > 0 {
            product *= n % 10;
            n /= 10;
        }
        n = product;
        count += 1;
    }
    count
}

#[cfg(test)]
mod tests {

    #[test]
    fn sample_tests() {
        assert_eq!(super::persistence(39), 3);
        assert_eq!(super::persistence(4), 0);
        assert_eq!(super::persistence(25), 2);
        assert_eq!(super::persistence(999), 4);
    }
}
