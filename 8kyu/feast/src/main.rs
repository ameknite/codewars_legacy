fn main() {}

fn feast(beast: &str, dish: &str) -> bool {
    let mut beast = beast.chars();
    let mut dish = dish.chars();
    beast.next() == dish.next() && beast.next_back() == dish.next_back()
}

#[test]
fn sample_test_cases() {
    assert_eq!(feast("great blue heron", "garlic naan"), true);
    assert_eq!(feast("chickadee", "chocolate cake"), true);
    assert_eq!(feast("brown bear", "bear claw"), false);
}
