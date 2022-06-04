fn main() {}

fn ordered_count(sip: &str) -> Vec<(char, i32)> {
    let mut vec: Vec<(char, i32)> = Vec::new();
    sip.chars().for_each(|x| {
        if let Some(y) = vec.iter_mut().find(|z| z.0 == x) {
            y.1 += 1;
        } else {
            vec.push((x, 1));
        }
    });
    vec
}

// fn ordered_count(sip: &str) -> Vec<(char, i32)> {
//     sip.chars().fold(Vec::new(), |mut acc, c| {
//         match acc.iter().position(|it| it.0 == c) {
//             Some(idx) => acc[idx].1 += 1,
//             None => acc.push((c, 1)),
//         };
//         acc
//     })
// }

// use itertools::Itertools;

// fn ordered_count(sip: &str) -> Vec<(char, i32)> {
//     sip
//         .chars()
//         .into_iter()
//         .unique()
//         .map(|c| (c, sip.matches(c).count() as i32))
//         .collect()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abradacadabra() {
        assert_eq!(
            ordered_count("abracadabra"),
            vec![('a', 5), ('b', 2), ('r', 2), ('c', 1), ('d', 1)]
        );
    }
    #[test]
    fn test_banana() {
        assert_eq!(ordered_count("banana"), vec![('b', 1), ('a', 3), ('n', 2)]);
    }
    #[test]
    fn test_master_solver() {
        assert_eq!(
            ordered_count("i am a master kata solver"),
            vec![
                ('i', 1),
                (' ', 5),
                ('a', 5),
                ('m', 2),
                ('s', 2),
                ('t', 2),
                ('e', 2),
                ('r', 2),
                ('k', 1),
                ('o', 1),
                ('l', 1),
                ('v', 1)
            ]
        );
    }
}
