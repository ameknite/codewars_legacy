fn main() {}

fn find_next_power(val: u64, pow_: u32) -> u64 {
    (1u64..).map(|x| x.pow(pow_)).find(|&x| x > val).unwrap()
}

#[cfg(test)]
mod tests {
    use super::find_next_power;

    fn dotest(n: u64, p: u32, expected: u64) {
        let actual = find_next_power(n, p);
        assert!(
            actual == expected,
            "With val = {n}, pow_ = {p}\nExpected {expected} but got {actual}"
        )
    }

    #[test]
    fn fixed_tests() {
        dotest(12385, 3, 13824);
        dotest(1245678, 5, 1419857);
        dotest(1245678, 6, 1771561);
    }
}
