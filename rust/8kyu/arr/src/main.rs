fn main() {}

fn arr(n: usize) -> Vec<u32> {
    (0..n.try_into().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::arr;

    #[test]
    fn example_tests() {
        assert_eq!(arr(0), vec![]);
        assert_eq!(arr(4), vec![0, 1, 2, 3]);
    }
}
