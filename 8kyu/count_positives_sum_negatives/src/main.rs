fn main() {}

fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    if input.is_empty() {
        return vec![];
    }
    vec![
        input.iter().filter(|x| x.is_positive()).count() as i32,
        input.iter().filter(|x| x.is_negative()).sum(),
    ]
}

#[test]
fn returns_expected() {
    let test_data1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14, -15];
    let expected1 = vec![10, -65];
    assert_eq!(count_positives_sum_negatives(test_data1), expected1);
}
