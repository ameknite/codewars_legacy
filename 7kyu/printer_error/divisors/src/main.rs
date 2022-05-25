fn main() {}

fn divisors(integer: u32) -> Result<Vec<u32>, String> {
    match (2..integer)
        .filter(|x| integer % x == 0)
        .collect::<Vec<_>>()
    {
        divisors if divisors.is_empty() => Err(format!("{integer} is prime")),
        divisors => Ok(divisors),
    }
}

// fn divisors(integer: u32) -> Result<Vec<u32>, String> {
//   let divisors: Vec<u32> = (2..integer / 2 + 1).filter(|x| integer % x == 0).collect();
//   match !divisors.is_empty() {
//     true => Ok(divisors),
//     _ => Err(format!("{} is prime", integer))
//   }
// }

#[test]
fn tests() {
    assert_eq!(divisors(15), Ok(vec![3, 5]));
    assert_eq!(divisors(12), Ok(vec![2, 3, 4, 6]));
    assert_eq!(divisors(13), Err("13 is prime".to_string()));
}
