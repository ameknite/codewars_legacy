fn main() {}

fn olympic_ring(s: &str) -> String {
    match s
        .chars()
        .map(|c| match c {
            'a' | 'b' | 'd' | 'e' | 'g' | 'o' | 'p' | 'q' | 'A' | 'D' | 'O' | 'P' | 'Q' | 'R' => 1,
            'B' => 2,
            _ => 0,
        })
        .sum::<i32>()
        / 2
    {
        x if x <= 1 => "Not even a medal!",
        2 => "Bronze!",
        3 => "Silver!",
        _ => "Gold!",
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::olympic_ring;

    #[test]
    fn basic() {
        assert_eq!(olympic_ring("wHjMudLwtoPGocnJ"), "Bronze!");
        assert_eq!(olympic_ring("eCEHWEPwwnvzMicyaRjk"), "Bronze!");
        assert_eq!(olympic_ring("JKniLfLW"), "Not even a medal!");
        assert_eq!(olympic_ring("EWlZlDFsEIBufsalqof"), "Silver!");
        assert_eq!(olympic_ring("IMBAWejlGRTDWetPS"), "Gold!");
    }
}
