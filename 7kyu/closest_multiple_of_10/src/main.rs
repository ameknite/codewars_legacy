fn main() {}

fn closest_multiple_of_10(n: u32) -> u32 {
    let remainder = n % 10;
    let n = n / 10 * 10;
    match remainder >= 5 {
        true => n + 10,
        false => n,
    }
}

// fn closest_multiple_of_10(n: u32) -> u32 {
//     (n + 5) / 10 * 10
// }

#[cfg(test)]
mod tests {
    use super::closest_multiple_of_10;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(n: u32, expected: u32) {
        assert_eq!(
            closest_multiple_of_10(n),
            expected,
            "{ERR_MSG} with n = {n}"
        )
    }

    #[test]
    fn fixed_tests() {
        dotest(54, 50);
        dotest(55, 60);
    }
}
