
fn main() {}

use std::collections::HashMap;

fn delete_nth(lst: &[u8], n: usize) -> Vec<u8> {
    let mut map = HashMap::with_capacity(lst.len());
    lst.iter()
        .filter(|&&x| {
            let count = map.entry(x).or_insert(0);
            *count += 1;
            *count <= n
        })
        .copied()
        .collect()
}

// fn delete_nth(xs: &[u8], n: usize) -> Vec<u8> {
//     let mut ks = [0; u8::MAX as usize + 1];
//     xs.iter().cloned()
//         .filter(|&x| { ks[x as usize] += 1; ks[x as usize] <= n })
//         .collect()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(delete_nth(&[20, 37, 20, 21], 1), vec![20, 37, 21]);
        assert_eq!(
            delete_nth(&[1, 1, 3, 3, 7, 2, 2, 2, 2], 3),
            vec![1, 1, 3, 3, 7, 2, 2, 2]
        );
    }
}
