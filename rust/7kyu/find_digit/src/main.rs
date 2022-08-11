fn main() {
    find_digit(5673, 4);
}

fn find_digit(num: i32, nth: i32) -> i32 {
    if !nth.is_positive() {
        return -1;
    }
    let mut num = num;
    let nth = nth as usize;
    let mut digits = vec![];
    if num.is_negative() {
        num = -num;
    }
    while num > 0 {
        digits.push(num % 10);
        num /= 10;
    }
    if nth > digits.len() {
        return 0;
    }
    println!("{:?}", digits);
    digits[nth - 1]
}

// fn find_digit(num: i32, nth: i32) -> i32 {
//     if nth <= 0 {
//         return -1;
//     }
//     let mut res = num;
//     for _ in 1..nth {
//         res = res / 10;
//     }
//     res.abs() % 10
// }

#[test]
fn example_test() {
    assert_eq!(find_digit(5673, 4), 5);
    assert_eq!(find_digit(129, 2), 2);
    assert_eq!(find_digit(-2825, 3), 8);
    assert_eq!(find_digit(-456, 4), 0);
    assert_eq!(find_digit(0, 20), 0);
    assert_eq!(find_digit(65, 0), -1);
    assert_eq!(find_digit(24, -8), -1);
}
