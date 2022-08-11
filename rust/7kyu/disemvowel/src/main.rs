fn main() {}

fn disemvowel(s: &str) -> String {
    s.matches(|x| !"aeiouAEIOU".contains(x)).collect()
}

#[cfg(test)]
mod tests {
    use super::disemvowel;

    #[test]
    fn example_test() {
        assert_eq!(
            disemvowel("This website is for losers LOL!"),
            "Ths wbst s fr lsrs LL!"
        );
    }
}
