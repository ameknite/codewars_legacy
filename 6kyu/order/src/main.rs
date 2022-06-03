fn main() {}

fn order(sentence: &str) -> String {
    let mut vec = sentence
        .split_whitespace()
        .map(|x| (x, x.chars().find(|x| x.is_numeric()).unwrap()))
        .collect::<Vec<_>>();
    vec.sort_by_key(|x| x.1);
    vec.iter().map(|x| x.0).collect::<Vec<_>>().join(" ")
}

// fn order(sentence: &str) -> String {
//     let mut ws: Vec<_> = sentence.split_whitespace().map(String::from).collect();
//     ws.sort_by_key(|s| s.chars().find(|c| c.is_numeric()).unwrap());
//     ws.join(" ")
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(order("is2 Thi1s T4est 3a"), "Thi1s is2 3a T4est");
        assert_eq!(order(""), "");
    }
}
