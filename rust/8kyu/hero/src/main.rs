fn main() {}

fn hero(bullets: u16, dragons: u16) -> bool {
    dragons * 2 <= bullets
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(hero(10, 5), true);
        assert_eq!(hero(7, 4), false);
        assert_eq!(hero(4, 5), false);
        assert_eq!(hero(100, 40), true);
        assert_eq!(hero(1500, 751), false);
        assert_eq!(hero(0, 1), false);
    }
}