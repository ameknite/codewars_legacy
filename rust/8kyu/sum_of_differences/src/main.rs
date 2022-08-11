fn main() {}

fn sum_of_differences(arr: &[i8]) -> Option<i8> {
    if arr.is_empty() || arr.len() == 1 {
        return None;
    }
    let mut arr = arr.to_vec();
    arr.sort_by(|a, b| b.cmp(a));
    Some(arr.windows(2).map(|w| w[0] - w[1]).sum::<i8>())
}

#[cfg(test)]
mod tests {
    use super::sum_of_differences;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    #[test]
    fn sample_tests() {
        assert_eq!(sum_of_differences(&[1, 2, 10]), Some(9), "{}", ERR_MSG);
        assert_eq!(sum_of_differences(&[-3, -2, -1]), Some(2), "{}", ERR_MSG);
        assert_eq!(sum_of_differences(&[1, 1, 1, 1, 1]), Some(0), "{}", ERR_MSG);
        assert_eq!(sum_of_differences(&[-17, 17]), Some(34), "{}", ERR_MSG);
        assert_eq!(sum_of_differences(&[]), None, "{}", ERR_MSG);
        assert_eq!(sum_of_differences(&[0]), None, "{}", ERR_MSG);
        assert_eq!(sum_of_differences(&[-1]), None, "{}", ERR_MSG);
        assert_eq!(sum_of_differences(&[1]), None, "{}", ERR_MSG);
    }
}
