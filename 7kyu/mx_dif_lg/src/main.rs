fn main() {}


fn mx_dif_lg(a1: Vec<&str>, a2: Vec<&str>) -> i32 {
    let mut vector = Vec::new();
    for x in a1 {
        for y in a2.iter() {
            vector.push((x.len() as i32 - y.len() as i32).abs());
        }
    }
    *vector.iter().max().unwrap_or(&-1)
}

// fn mx_dif_lg(a1: Vec<&str>, a2: Vec<&str>) -> i32 {
//     // your code
//     a1.iter().flat_map(|s1| a2.iter().map(|s2| (s1.len() as i32 - s2.len() as i32).abs()).collect::<Vec<_>>() ).max().unwrap_or(-1)
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(a1: Vec<&str>, a2: Vec<&str>, exp: i32) -> () {
        println!("a1: {:?};", a1);
        println!("a2: {:?};", a2);
        let ans = mx_dif_lg(a1, a2);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!("{};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        let mut s1 = vec![
            "hoqq",
            "bbllkw",
            "oox",
            "ejjuyyy",
            "plmiis",
            "xxxzgpsssa",
            "xxwwkktt",
            "znnnnfqknaz",
            "qqquuhii",
            "dvvvwz",
        ];
        let mut s2 = vec!["cccooommaaqqoxii", "gggqaffhhh", "tttoowwwmmww"];
        dotest(s1, s2, 13);
        s1 = vec![
            "ejjjjmmtthh",
            "zxxuueeg",
            "aanlljrrrxx",
            "dqqqaaabbb",
            "oocccffuucccjjjkkkjyyyeehh",
        ];
        s2 = vec!["bbbaaayddqbbrrrv"];
        dotest(s1, s2, 10);
    }
}
