fn main() {}

fn even_numbers(array: &Vec<i32>, number: usize) -> Vec<i32> {
    let vec = array
        .iter()
        .filter(|&&x| x % 2 == 0)
        .copied()
        .collect::<Vec<_>>();
    vec[vec.len() - number..].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(
            even_numbers(&vec!(1, 2, 3, 4, 5, 6, 7, 8, 9), 3),
            vec!(4, 6, 8)
        );
        assert_eq!(
            even_numbers(&vec!(-22, 5, 3, 11, 26, -6, -7, -8, -9, -8, 26), 2),
            vec!(-8, 26)
        );
        assert_eq!(
            even_numbers(&vec!(6, -25, 3, 7, 5, 5, 7, -3, 23), 1),
            vec!(6)
        );
    }
}
