fn main() {}

fn fib(mut n: u32) -> u32 {
    let (mut a, mut b) = (0, 1);
    while n != 0 {
        (a, b) = (b, a + b);
        n -= 1;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::fib;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(n: u32, expected: u32) {
        assert_eq!(fib(n), expected, "{ERR_MSG} with n = {n}")
    }

    #[test]
    fn returns_expected() {
        dotest(0, 0);
        dotest(1, 1);
        dotest(2, 1);
        dotest(3, 2);
        dotest(4, 3);
        dotest(5, 5);
    }
}
