fn main() {}

fn usdcny(usd: u16) -> String {
    format!("{:.2} Chinese Yuan", f64::from(usd) * 6.75)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(usdcny(15), "101.25 Chinese Yuan");
        assert_eq!(usdcny(465), "3138.75 Chinese Yuan");
    }
}
