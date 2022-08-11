fn main() {
    println!("{}", dig_pow(89, 1))
}

fn dig_pow(n: i64, p: i32) -> i64 {
    let sum = n.to_string().chars().enumerate().fold(0, |acc, (i, c)| {
        acc + (i64::from(c.to_digit(10).unwrap())).pow(i as u32 + p as u32)
    });
    match sum % n == 0 {
        true => sum / n,
        false => -1,
    }
}

// fn dig_pow(n: i64, p: i32) -> i64 {
//     let mut sm: i64 = 0;
//     let mut m = n;
//     let mut pp: u32 = ((n as f64).log10() as u32) + (p as u32);
//     while m > 0 {
//         sm += (m % 10).pow(pp);
//         m = m / 10;
//         pp -= 1;
//     }
//     if sm % n == 0 {
//         return sm / n;
//     } else {
//         return -1;
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(n: i64, p: i32, exp: i64) -> () {
        println!(" n: {:?};", n);
        println!("p: {:?};", p);
        let ans = dig_pow(n, p);
        println!(" actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!(" {};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(89, 1, 1);
        dotest(92, 1, -1);
        dotest(46288, 3, 51);
    }
}
