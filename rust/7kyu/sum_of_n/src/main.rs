fn main() {}

fn sum_of_n(n: i32) -> Vec<i32> {
    match n.is_positive() {
        true => (0..=n).map(|x| (0..=x).sum::<i32>()).collect(),
        false => (0..=-n).map(|x| -(0..=x).sum::<i32>()).collect(),
    }
}



#[test]
fn normal_tests() {
    assert_eq!(sum_of_n(3), vec![0, 1, 3, 6]);
    assert_eq!(sum_of_n(-4), vec![0, -1, -3, -6, -10]);
    assert_eq!(sum_of_n(1), vec![0, 1]);
    assert_eq!(sum_of_n(0), vec![0]);
    assert_eq!(sum_of_n(10), vec![0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55]);
}
