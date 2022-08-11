
use itertools::Itertools;

fn main() {}

fn flip(dir: char, cubes: &[u32]) -> Vec<u32> {
    match dir {
        'R' => cubes.iter().sorted().map(|&x| x).collect(),
        'L' => cubes.iter().sorted().rev().map(|&x| x).collect(),
        _ => panic!()
    }
}


// fn flip(dir: char, cubes: &[u32]) -> Vec<u32> {
//     let mut arr = cubes.to_vec();
//     match dir {
//         'R' => arr.sort_by(|a, b| a.cmp(b)),
//         _ => arr.sort_by(|a, b| b.cmp(a))

//     };
//     arr
// }

#[cfg(test)]
mod tests {
    use super::flip;

    #[test]
    fn sample_tests() {
        assert_eq!(flip('R', &vec![3, 2, 1, 2]), vec![1, 2, 2, 3]);
        assert_eq!(flip('L', &vec![1, 4, 5, 3, 5]), vec![5, 5, 4, 3, 1]);
    }
}
