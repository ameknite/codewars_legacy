fn main() {}

fn greet(language: &str) -> &str {
    match language {
        "english" => "Welcome",
        "czech" => "Vitejte",
        "danish" => "Velkomst",
        "dutch" => "Welkom",
        "estonian" => "Tere tulemast",
        "finnish" => "Tervetuloa",
        "flemish" => "Welgekomen",
        "french" => "Bienvenue",
        "german" => "Willkommen",
        "irish" => "Failte",
        "italian" => "Benvenuto",
        "latvian" => "Gaidits",
        "lithuanian" => "Laukiamas",
        "polish" => "Witamy",
        "spanish" => "Bienvenido",
        "swedish" => "Valkommen",
        "welsh" => "Croeso",
        _ => "Welcome",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filed() {
        assert_eq!(greet("english"), "Welcome");
        assert_eq!(greet("dutch"), "Welkom");
        assert_eq!(greet("IP_ADDRESS_INVALID"), "Welcome");
        assert_eq!(greet(""), "Welcome");
        assert_eq!(greet("swelsh"), "Welcome");
    }
}
