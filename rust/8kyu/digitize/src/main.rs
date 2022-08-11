fn main() {
    println!("Hello, world!");
}

fn digitize(n: u64) -> Vec<u8> {
    n.to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u8)
        .rev()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed() {
        assert_eq!(digitize(35231), vec![1, 3, 2, 5, 3]);
        assert_eq!(digitize(0), vec![0]);
    }
}
