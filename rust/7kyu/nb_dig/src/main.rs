fn main() {
    nb_dig(25, 1);
}

fn nb_dig(n: i32, d: i32) -> i32 {
    (0..=n)
        .map(|x| x.pow(2).to_string())
        .collect::<String>()
        .chars()
        .filter(|&x| x.to_digit(10).unwrap() == d.try_into().unwrap())
        .count() as i32
}

// fn nb_dig(n: i32, d: i32) -> i32 {
//     let ex = d.to_string();
//     (0..=n).map(|v| (v*v).to_string()).fold(0, |acc, v| acc + v.matches(&ex).count() as i32)
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(n: i32, d: i32, exp: i32) -> () {
        println!("n: {:?}", n);
        println!("d: {:?}", d);
        let ans = nb_dig(n, d);
        println!("actual:\n{:?}", ans);
        println!("expect:\n{:?}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(550, 5, 213);
        dotest(5750, 0, 4700);
    }
}
