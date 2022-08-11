fn main() {}

fn unique_in_order<T>(sequence: T) -> Vec<T::Item>
where
    T: std::iter::IntoIterator,
    T::Item: std::cmp::PartialEq + std::fmt::Debug,
{
    sequence.into_iter().fold(vec![], |mut acc, x| {
        if acc.last() != Some(&x) {
            acc.push(x);
        }
        acc
    })
}

// fn unique_in_order<T>(sequence: T) -> Vec<T::Item>
// where
//     T: std::iter::IntoIterator,
//     T::Item: std::cmp::PartialEq + std::fmt::Debug,
// {
//     let mut v: Vec<_> = Vec::from_iter(sequence);
//     v.dedup();
//     v
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test() {
        assert_eq!(
            unique_in_order("AAAABBBCCDAABBB".chars()),
            vec!['A', 'B', 'C', 'D', 'A', 'B']
        );
    }
}
