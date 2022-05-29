fn main() {}

fn check_exam(arr_a: &[&str], arr_b: &[&str]) -> i64 {
    let result = arr_a
        .iter()
        .zip(arr_b.iter())
        .map(|(&a, &b)| match b {
            b if b == a => 4,
            b if b == "" => 0,
            _ => -1,
        })
        .sum();
    match result < 0 {
        true => 0,
        false => result,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(check_exam(&["a", "a", "b", "b"], &["a", "c", "b", "d"]), 6);
        assert_eq!(check_exam(&["a", "a", "c", "b"], &["a", "a", "b", ""]), 7);
        assert_eq!(check_exam(&["a", "a", "b", "c"], &["a", "a", "b", "c"]), 16);
        assert_eq!(check_exam(&["b", "c", "b", "a"], &["", "a", "a", "c"]), 0);
    }
}
