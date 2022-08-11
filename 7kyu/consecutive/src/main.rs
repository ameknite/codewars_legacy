fn main() {}

fn consecutive(xs: &[i16]) -> i16 {
    if xs.is_empty() {
        return 0;
    };
    xs.iter().max().unwrap() - xs.iter().min().unwrap() - xs.len() as i16 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(consecutive(&[4, 8, 6]), 2);
        assert_eq!(consecutive(&[1, 2, 3, 4]), 0);
        assert_eq!(consecutive(&[]), 0);
        assert_eq!(consecutive(&[1]), 0);
    }
}
