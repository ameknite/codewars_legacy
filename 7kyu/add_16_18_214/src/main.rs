fn main() {}

fn add(num1: u32, num2: u32) -> u64 {
    if num1 == 0 && num2 == 0 {
        return 0;
    }
    let mut num1 = num1;
    let mut num2 = num2;
    let mut vec = Vec::new();
    while num1 > 0 || num2 > 0 {
        vec.push(num1 % 10 + num2 % 10);
        num1 /= 10;
        num2 /= 10;
    }
    vec.iter()
        .rev()
        .fold("".to_string(), |acc, x| acc + &x.to_string())
        .parse()
        .unwrap()
}

// fn add(mut x: u32, mut y: u32) -> u64 {
//     let mut res = 0;
//     let mut m = 1;
//     while x != 0 || y != 0 {
//         let s = x % 10 + y % 10;
//         res += s as u64 * m;
//         m *= if s < 10 { 10 } else { 100 };
//         x /= 10;
//         y /= 10;
//     }
//     res
// }

#[test]
fn test_real_add() {
    // behaves just like normal addition in these cases
    assert_eq!(add(2, 11), 13);
    assert_eq!(add(0, 1), 1);
    assert_eq!(add(0, 0), 0);
}

#[test]
fn test_silly_add() {
    // should also work with the "new", silly addition method
    assert_eq!(add(16, 18), 214);
    assert_eq!(add(26, 39), 515);
    assert_eq!(add(122, 81), 1103);
}
