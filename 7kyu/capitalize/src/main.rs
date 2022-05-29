fn main() {}

fn capitalize(s: &str) -> Vec<String> {
    let mut first = String::with_capacity(s.len());
    let mut second = String::with_capacity(s.len());
    s.char_indices().for_each(|(i, c)| match i % 2 == 0 {
        true => {
            first.extend(c.to_uppercase());
            second.extend(c.to_lowercase());
        }
        false => {
            first.extend(c.to_lowercase());
            second.extend(c.to_uppercase());
        }
    });
    vec![first, second]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_tests() {
        assert_eq!(capitalize("abcdef"), ["AbCdEf", "aBcDeF"]);
        assert_eq!(capitalize("codewars"), ["CoDeWaRs", "cOdEwArS"]);
        assert_eq!(capitalize("abracadabra"), ["AbRaCaDaBrA", "aBrAcAdAbRa"]);
        assert_eq!(capitalize("codewarriors"), ["CoDeWaRrIoRs", "cOdEwArRiOrS"]);
    }
}
