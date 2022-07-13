fn main() {}

// fn calc(s: &str) -> u32 {
//     let (total1, total2) = s.chars().map(|c| u32::from(c).to_string()).fold(
//         (String::new(), String::new()),
//         |(t1, t2), elem| {
//             let new_elem: String = elem
//                 .chars()
//                 .map(|c| match c {
//                     '7' => '1',
//                     _ => c,
//                 })
//                 .collect();
//             (t1 + &elem, t2 + &new_elem)
//         },
//     );

//     total1.chars().flat_map(|x| x.to_digit(10)).sum::<u32>()
//         - total2.chars().flat_map(|x| x.to_digit(10)).sum::<u32>()
// }
fn calc(s: &str) -> u32 {
    s.chars()
        .map(|c| match c {
            'C' | 'F'..='L' | 'N' | 'O' | 'W' | 'a' | 'k' | 'u' => 1,
            'M' => 2,
            _ => 0,
        })
        .sum::<u32>()
        * 6
}

// fn calc(s: &str) -> u32 {
//     s.chars()
//         .map(|c| {
//             u32::from(c)
//                 .to_string()
//                 .chars()
//                 .filter(|&c| c == '7')
//                 .count() as u32
//         })
//         .sum::<u32>()
//         * 6
// }

#[cfg(test)]
mod tests {
    use super::calc;

    #[test]
    fn example_tests() {
        // do_test("ABC", 6);
        do_test("abcdef", 6);
        // do_test("ifkhchlhfd", 6);
        // do_test("aaaaaddddr", 30);
        // do_test("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ", 96);
    }

    fn do_test(input: &str, expected: u32) {
        let user_result = calc(input);
        assert!(
            user_result == expected,
            "Test Failed!\ncalc(\"{}\") was {}\nExpected {}",
            input,
            user_result,
            expected
        );
    }
}
