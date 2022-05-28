fn main() {}

fn evaporator(content: f64, evap_per_day: i32, threshold: i32) -> i32 {
    let mut days = 0;
    let threshold = content * (f64::from(threshold) / 100.0);
    let mut content = content;
    while content > threshold {
        content -= content * (f64::from(evap_per_day) / 100.0);
        days += 1;
    }
    days
}

// fn evaporator(_content: f64, evap_per_day: i32, threshold: i32) -> i32 {
//     // content_day = content_0*(1 - evap_per_day)^day
//     // content_day/content_0 = (1 - evap_per_day)^day
//     // threshold = (1 - evap_per_day)^day
//     // log(threshold) = day*log(1 - evap_per_day)
//     // day = log(threshold)/log(1 - evap_per_day)
//     let evap_per_day = evap_per_day as f64/100.0;
//     let threshold = threshold as f64/100.0;
//     (threshold.log2()/(1.0 - evap_per_day).log2()).ceil() as i32
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(_content: f64, evap_per_day: i32, threshold: i32, exp: i32) -> () {
        println!(" evap_per_day: {:?}", evap_per_day);
        println!("threshold: {:?}", threshold);
        let ans = evaporator(_content, evap_per_day, threshold);
        println!(" actual:\n{:?}", ans);
        println!("expect:\n{:?}", exp);
        println!(" {};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(10.0, 10, 10, 22);
        dotest(10.0, 10, 5, 29);
    }
}
