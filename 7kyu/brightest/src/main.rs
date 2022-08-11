fn main() {}

fn brightest(colors: &[String]) -> String {
    colors
        .iter()
        .map(|color| {
            let r = u8::from_str_radix(&color[1..=2], 16).unwrap();
            let g = u8::from_str_radix(&color[3..=4], 16).unwrap();
            let b = u8::from_str_radix(&color[5..], 16).unwrap();
            let value = r.max(g).max(b);
            (value, color)
        })
        .rev()
        .max_by_key(|&(v, _)| v)
        .unwrap()
        .1
        .to_owned()
}

// fn brightest(colors: &[String]) -> String {
//     colors
//         .iter()
//         .rev()
//         .max_by_key(|c| c[1..].as_bytes().chunks_exact(2).max())
//         .unwrap()
//         .clone()
// }

#[cfg(test)]
mod tests {
    use super::brightest;

    fn dotest(colors: &[String], expected: &str) {
        let actual = brightest(colors);
        assert!(
            actual == expected,
            "With colors = {colors:?}\nExpected \"{expected}\" but got \"{actual}\""
        )
    }

    #[test]
    fn fixed_tests() {
        dotest(
            &[String::from("#001000"), String::from("#000000")],
            "#001000",
        );
        dotest(
            &[String::from("#ABCDEF"), String::from("#123456")],
            "#ABCDEF",
        );
        dotest(
            &[String::from("#00FF00"), String::from("#FFFF00")],
            "#00FF00",
        );
        dotest(
            &[String::from("#FFFFFF"), String::from("#1234FF")],
            "#FFFFFF",
        );
        dotest(
            &[
                String::from("#FFFFFF"),
                String::from("#123456"),
                String::from("#000000"),
            ],
            "#FFFFFF",
        );
    }
}
