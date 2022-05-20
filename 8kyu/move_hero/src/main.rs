fn main() {}

fn move_hero(position: u32, roll: u32) -> u32 {
    roll * 2 + position
}

#[cfg(test)]
mod tests {
    use super::move_hero;

    #[test]
    fn sample_tests() {
        assert_eq!(move_hero(0, 4), 8);
    }
}
