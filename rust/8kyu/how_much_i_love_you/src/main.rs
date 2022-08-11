fn main() {
    println!("Hello, world!");
}

fn how_much_i_love_you(nb_petals: u16) -> &'static str {
    println!("{}", nb_petals);
    match nb_petals % 6 {
        1 => "I love you",
        2 => "a little",
        3 => "a lot",
        4 => "passionately",
        5 => "madly",
        0 => "not at all",
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::how_much_i_love_you;

    #[test]
    fn fixed_tests() {
        assert_eq!(how_much_i_love_you(7), "I love you");
        assert_eq!(how_much_i_love_you(3), "a lot");
        assert_eq!(how_much_i_love_you(6), "not at all");
    }
}
