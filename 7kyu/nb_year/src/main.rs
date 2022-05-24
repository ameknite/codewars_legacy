fn main() {}

fn nb_year(p0: i32, percent: f64, aug: i32, p: i32) -> i32 {
    let mut percent_value = f64::from(p0) * (percent / 100.0);
    let mut p0 = p0;
    let mut steps = 0;
    while p0 < p {
        steps += 1;
        p0 = (f64::from(p0) + percent_value + f64::from(aug)) as i32;
        percent_value = f64::from(p0) * (percent / 100.0);
    }
    steps
}

// fn nb_year(p0: i32, percent: f64, aug: i32, p: i32)-> i32 {
//     std::iter::successors(Some(p0), |&p0|
//         Some(p0 + (p0 as f64 * percent / 100.0) as i32 + aug)
//     ).take_while(|&p0| p0 < p).count() as i32
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(p0: i32, percent: f64, aug: i32, p: i32, exp: i32) -> () {
        println!("p0: {:?};", p0);
        println!("percent: {:?};", percent);
        println!("aug: {:?};", aug);
        println!("p: {:?};", p);
        let ans = nb_year(p0, percent, aug, p);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!("{};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(1500, 5.0, 100, 5000, 15);
        dotest(1500000, 2.5, 10000, 2000000, 10);
    }
}
