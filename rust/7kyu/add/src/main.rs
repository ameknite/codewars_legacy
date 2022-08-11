fn main() {}

fn add(args: &[i64]) -> i64 {
    args.iter().zip(1..).map(|(&x, y)| x * y).sum()
}

#[test]
fn basic_tests() {
    assert_eq!(add(&[]), 0);
    assert_eq!(add(&[4, -3, -2]), -8);
}
