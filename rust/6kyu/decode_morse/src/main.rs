fn main() {}

mod preloaded;
use preloaded::MORSE_CODE;
// MORSE_CODE is `HashMap<String, String>`. e.g. ".-" -> "A".

fn decode_morse(encoded: &str) -> String {
    encoded
        .split("   ")
        .map(|word| {
            word.split(" ")
                .map(|letter| {
                    MORSE_CODE
                        .get(letter)
                        .unwrap_or(&"".to_string())
                        .to_string()
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hey_jude() {
        assert_eq!(decode_morse(".... . -.--   .--- ..- -.. ."), "HEY JUDE");
    }
}
