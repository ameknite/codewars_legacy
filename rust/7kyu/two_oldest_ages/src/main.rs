fn main() {}

fn two_oldest_ages(ages: &[u8]) -> [u8; 2] {
    let max = *ages.iter().max().unwrap();
    let mut ages = ages.to_vec();
    ages.remove(ages.iter().position(|&x| x == max).unwrap());
    [*ages.iter().max().unwrap(), max]
}

// fn two_oldest_ages(ages: &[u8]) -> [u8; 2] {
//     let mut ages = ages.to_vec();
//     ages.sort();

//     [ages[ages.len() - 2], ages[ages.len() - 1]]
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_oldest_ages() {
        assert_eq!(two_oldest_ages(&[1, 5, 87, 45, 8, 8]), [45, 87]);
        assert_eq!(two_oldest_ages(&[6, 5, 83, 5, 3, 18]), [18, 83]);
        assert_eq!(two_oldest_ages(&[10, 1]), [1, 10]);
        assert_eq!(two_oldest_ages(&[1, 3, 10, 0]), [3, 10]);
    }
}
