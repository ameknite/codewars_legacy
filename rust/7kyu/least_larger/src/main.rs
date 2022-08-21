fn main() {}

fn least_larger(a: &[i32], i: usize) -> Option<usize> {
    a.iter()
        .enumerate()
        .filter(|(_, &x)| x > a[i])
        .min_by_key(|(_, &x)| x)
        .map(|(i, _)| i)
}

#[cfg(test)]
mod tests {
    use super::least_larger;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(a: &[i32], i: usize, expected: Option<usize>) {
        assert_eq!(
            least_larger(a, i),
            expected,
            "{ERR_MSG} with a = {a:?}, i = {i}"
        )
    }

    #[test]
    fn sample_tests() {
        dotest(&[4, 1, 3, 5, 6], 0, Some(3));
        dotest(&[4, 1, 3, 5, 6], 4, None);
        dotest(&[1, 3, 5, 2, 4], 0, Some(3));
    }
}
