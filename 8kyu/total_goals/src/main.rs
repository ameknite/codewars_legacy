fn main() {}

const la_liga_goals: u32 = 43;
const champions_league_goals: u32 = 10;
const copa_del_rey_goals: u32 = 5;

const total_goals: u32 = la_liga_goals + champions_league_goals + copa_del_rey_goals;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(total_goals, 58, "total goals should equal to 58");
    }
}
