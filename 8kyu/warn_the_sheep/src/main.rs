fn main() {}

fn warn_the_sheep(queue: &[&str]) -> String {
    let position = queue.iter().rev().position(|&x| x == "wolf").unwrap();
    match position {
        0 => "Pls go away and stop eating my sheep".to_string(),
        _ => format!("Oi! Sheep number {position}! You are about to be eaten by a wolf!"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(
            warn_the_sheep(&[
                "sheep", "sheep", "sheep", "sheep", "sheep", "wolf", "sheep", "sheep"
            ]),
            "Oi! Sheep number 2! You are about to be eaten by a wolf!",
        );
        assert_eq!(
            warn_the_sheep(&["sheep", "wolf", "sheep", "sheep", "sheep", "sheep", "sheep"]),
            "Oi! Sheep number 5! You are about to be eaten by a wolf!",
        );
        assert_eq!(
            warn_the_sheep(&["wolf", "sheep", "sheep", "sheep", "sheep", "sheep", "sheep"]),
            "Oi! Sheep number 6! You are about to be eaten by a wolf!",
        );
        assert_eq!(
            warn_the_sheep(&["sheep", "wolf", "sheep"]),
            "Oi! Sheep number 1! You are about to be eaten by a wolf!",
        );
        assert_eq!(
            warn_the_sheep(&["sheep", "sheep", "wolf"]),
            "Pls go away and stop eating my sheep",
        );
    }
}
