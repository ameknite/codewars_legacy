fn main() {}

fn arrow_area(a: u32, b: u32) -> f64 {
    let a: f64 = a.into();
    let b: f64 = b.into();
    a * b / 4.0
}

#[cfg(test)]
mod tests {
    use super::arrow_area;

    fn dotest(a: u32, b: u32, expected: f64) {
        let actual = arrow_area(a, b);
        assert!(
            actual == expected,
            "With a = {a}, b = {b}\nExpected {expected} but got {actual}"
        )
    }

    #[test]
    fn fixed_tests() {
        dotest(4, 2, 2.0);
        dotest(7, 6, 10.5);
        dotest(25, 25, 156.25);
    }
}
