fn main() {
}

fn subtract_sum(_: u32) -> &'static str {
    "apple"
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(subtract_sum(10), "apple");
    }
}
