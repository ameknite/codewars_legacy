fn main() {}

fn next_id(ids: &[usize]) -> usize {
    let max = match ids.iter().max() {
        Some(x) => *x,
        None => 0,
    };
    match (0..=max).find(|x| !ids.contains(&x)) {
        Some(x) => x,
        None => max + 1,
    }
}

// fn next_id(ids: &[usize]) -> usize {
//     (0..).find(|n| !ids.contains(&n)).unwrap()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basics() {
        assert_eq!(next_id(&[0, 1, 2, 4, 5]), 3);
        assert_eq!(next_id(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 11);
        assert_eq!(next_id(&[]), 0);
    }
}
