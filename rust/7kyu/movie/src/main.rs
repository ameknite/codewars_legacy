fn main() {}

fn movie(card: i32, ticket: i32, perc: f64) -> i32 {
    let card: f64 = card.into();
    let ticket: f64 = ticket.into();
    let mut p = ticket * perc;
    let mut b = card + p;
    for i in 2.. {
        let a = ticket * f64::from(i);
        b += p * perc;
        p *= perc;
        if a > b.ceil() {
            return i;
        }
    }
    0
}

// fn movie(card: i32, ticket: i32, perc: f64) -> i32 {
//     let mut sum = card as f64;
//     let ticket = ticket as f64;
//     for n in 1.. {
//         sum += ticket * perc.powf(n as f64);
//         if sum.ceil() < ticket * n as f64 {
//             return n;
//         }
//     }
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(card: i32, ticket: i32, perc: f64, exp: i32) -> () {
        println!(" card: {:?};", card);
        println!("ticket: {:?};", ticket);
        println!("perc: {:?};", perc);
        let ans = movie(card, ticket, perc);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!(" {};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(500, 15, 0.9, 43);
        dotest(100, 10, 0.95, 24);
        dotest(0, 10, 0.95, 2);
    }
}
