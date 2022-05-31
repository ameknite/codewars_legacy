fn main() {}

fn cooking_time(eggs: u32) -> u32 {
    (0..eggs).step_by(8).count() as u32 * 5
}

// fn cooking_time(eggs: u32) -> u32 {
//     (eggs + 7) / 8 * 5
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_tests() {
        do_test(0, 0);
        do_test(5, 5);
        do_test(10, 10);
    }

    fn do_test(eggs: u32, exp: u32) {
        let user_time = cooking_time(eggs);
        assert!(
            user_time == exp,
            "Test failed!\ncooking_time({}) was {}\nExpected {}",
            eggs,
            user_time,
            exp,
        );
    }
}
