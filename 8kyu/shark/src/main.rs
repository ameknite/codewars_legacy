fn main() {}

fn shark(
    pontoon_distance: f64,
    shark_distance: f64,
    you_speed: f64,
    shark_speed: f64,
    dolphin: bool,
) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(shark(12.0, 50.0, 4.0, 8.0, true), "Alive!");
        assert_eq!(shark(7.0, 55.0, 4.0, 16.0, true), "Alive!");
        assert_eq!(shark(24.0, 0.0, 4.0, 8.0, true), "Shark Bait!");
        assert_eq!(shark(40.0, 35.0, 3.0, 20.0, true), "Shark Bait!");
        assert_eq!(shark(7.0, 8.0, 3.0, 4.0, true), "Alive!");
    }
}
