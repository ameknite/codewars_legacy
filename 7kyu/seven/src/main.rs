fn main() {
    println!("{:?}", seven(371))
}
#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(n: i64, exp: (i64, i32)) -> () {
        println!(" n: {:?};", n);
        let ans = seven(n);
        println!(" actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!(" {};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(477557101, (28, 7));
        dotest(477557102, (47, 7));
        dotest(1603, (7, 2));
    }
}

fn seven(n: i64) -> (i64, i32) {
    let mut n = n;
    let mut count = 0;
    while n > 99 {
        n = n / 10 - n % 10 * 2;
        count += 1;
    }
    (n, count)
}
