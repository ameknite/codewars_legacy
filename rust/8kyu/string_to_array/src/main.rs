fn main() {}

fn string_to_array(s: &str) -> Vec<String> {
    s.split_whitespace().map(str::to_owned).collect()
}

#[cfg(test)]
mod tests {
    use super::string_to_array;

    #[test]
    fn fixed_tests() {
        assert_eq!(
            string_to_array("Robin Singh"),
            ["Robin", "Singh"],
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            string_to_array("CodeWars"),
            ["CodeWars"],
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            string_to_array("I love arrays they are my favorite"),
            ["I", "love", "arrays", "they", "are", "my", "favorite"],
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            string_to_array("1 2 3"),
            ["1", "2", "3"],
            "\nYour answer (left) is not the expected answer (right)."
        );
    }
}
