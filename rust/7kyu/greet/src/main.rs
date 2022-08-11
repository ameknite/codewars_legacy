fn main() {}

struct Person<'a> {
    name: &'a str,
}

impl Person<'_> {
    fn new(name: &str) -> Person {
        Person { name }
    }

    fn greet(&self, other: &str) -> String {
        format!("Hello {}, my name is {}", other, self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::Person;

    #[test]
    fn greet() {
        let alice = Person::new("Alice");
        assert_eq!(alice.name, "Alice");
        assert_eq!(alice.greet("Bob"), "Hello Bob, my name is Alice");
    }

    #[test]
    fn name() {
        let charlie = Person::new("Charlie");
        assert_eq!(charlie.name, "Charlie");
    }
}
