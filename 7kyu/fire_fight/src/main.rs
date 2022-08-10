fn main() {}

fn fire_fight(s: &str) -> String {
    s.replace("Fire", "~~")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            fire_fight("Boat Rudder Mast Boat Hull Water Fire Boat Deck Hull Fire Propeller Deck Fire Deck Boat Mast"),
            "Boat Rudder Mast Boat Hull Water ~~ Boat Deck Hull ~~ Propeller Deck ~~ Deck Boat Mast");

        assert_eq!(
            fire_fight("Mast Deck Engine Water Fire"),
            "Mast Deck Engine Water ~~"
        );

        assert_eq!(
            fire_fight(
                "Fire Deck Engine Sail Deck Fire Fire Fire Rudder Fire Boat Fire Fire Captain"
            ),
            "~~ Deck Engine Sail Deck ~~ ~~ ~~ Rudder ~~ Boat ~~ ~~ Captain"
        );
    }
}
