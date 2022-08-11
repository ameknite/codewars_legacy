fn main() {}

fn hello(name: &str) -> String {
    println!("{name}");
    let mut chars = name.chars();
    let name = match chars.next() {
        Some(c) => {
            c.to_uppercase().to_string() + &chars.as_str().to_lowercase()
        }
        None => "World".to_string(),
    };
    format!("Hello, {name}!",)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(hello("johN"), "Hello, John!");
        assert_eq!(hello("alice"), "Hello, Alice!");
        assert_eq!(hello(""), "Hello, World!");
    }
}
