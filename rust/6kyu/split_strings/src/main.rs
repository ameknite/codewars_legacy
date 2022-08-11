fn main() {}

fn solution(s: &str) -> Vec<String> {
    let mut solution = Vec::new();
    let s = s.chars().collect::<Vec<_>>();
    let chunks = s.chunks_exact(2);
    let mut last_chunk = chunks.remainder().to_vec();
    solution.extend(chunks.map(|chunk| chunk.iter().collect()));
    if last_chunk.len() == 0 {
        return solution;
    }
    last_chunk.push('_');
    solution.push(last_chunk.iter().collect());
    solution
}

// fn solution(s: &str) -> Vec<String> {
//     s.chars()
//         .collect::<Vec<_>>()
//         .chunks(2)
//         .map(|v| {
//             if v.len() == 1 {
//                 format!("{}_", v[0])
//             } else {
//                 v.into_iter().collect()
//             }
//         })
//         .collect()
// }

// use itertools::Itertools;

// fn solution(s: &str) -> Vec<String> {
//     s.chars().chunks(2).into_iter().map(|c| c.pad_using(2, |_| '_').collect()).collect()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(solution("abcdef"), ["ab", "cd", "ef"]);
        assert_eq!(solution("abcdefg"), ["ab", "cd", "ef", "g_"]);
        assert_eq!(solution(""), [] as [&str; 0]);
    }
}
