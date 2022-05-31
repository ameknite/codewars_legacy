fn main() {}

fn part_list(arr: Vec<&str>) -> String {
    (0..arr.len() - 1)
        .map(|x| {
            format!(
                "({}{}{})",
                if x == 0 {
                    "".to_owned()
                } else {
                    arr[..x].join(" ") + " "
                },
                arr[x].to_owned() + ", ",
                arr[x + 1..].join(" "),
            )
        })
        .collect()
}

// fn part_list(arr: Vec<&str>) -> String {
//     let n = arr.len();
//     (1..n)
//         .map(|i| format!("({}, {})", arr[0..i].join(" "), arr[i..].join(" ")))
//         .collect::<Vec<_>>()
//         .join("")
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(arr: Vec<&str>, exp: &str) -> () {
        println!("arr: {:?}", arr);
        let ans = part_list(arr);
        println!("actual:\n{}", ans);
        println!("expect:\n{}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }

    #[test]
    fn basis_tests() {
        dotest(vec!["I", "wish", "I", "hadn't", "come"],
                "(I, wish I hadn't come)(I wish, I hadn't come)(I wish I, hadn't come)(I wish I hadn't, come)");
        dotest(
            vec!["cdIw", "tzIy", "xDu", "rThG"],
            "(cdIw, tzIy xDu rThG)(cdIw tzIy, xDu rThG)(cdIw tzIy xDu, rThG)",
        );
    }
}
