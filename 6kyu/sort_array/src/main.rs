fn main() {}

fn sort_array(arr: &[i32]) -> Vec<i32> {
    let mut odds: Vec<i32> = arr.iter().filter(|&&x| x % 2 == 1).copied().collect();
    odds.sort_by(|a, b| b.cmp(a));
    arr.iter()
        .map(|&x| match x % 2 {
            0 => x,
            _ => odds.pop().unwrap(),
        })
        .collect()
}

// use itertools::Itertools;

// fn sort_array(xs: &[i32]) -> Vec<i32> {
//     let mut os = xs.iter().filter(|&x| x % 2 != 0).sorted();
//     xs.iter().map(|x| if x % 2 != 0 { os.next().unwrap() } else { x }).cloned().collect()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(sort_array(&[5, 3, 2, 8, 1, 4]), [1, 3, 2, 8, 5, 4]);
        assert_eq!(sort_array(&[5, 3, 1, 8, 0]), [1, 3, 5, 8, 0]);
        assert_eq!(sort_array(&[]), []);
    }
}
