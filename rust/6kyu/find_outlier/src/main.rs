fn main() {}

fn find_outlier(values: &[i32]) -> i32 {
    if values.iter().filter(|&&i| i % 2 == 0).count() == 1 {
        return *values.iter().find(|&i| i % 2 == 0).unwrap();
    } else {
        return *values.iter().find(|&i| i % 2 != 0).unwrap();
    }
}

// fn find_outlier(list: &[i32]) -> i32 {
//     let parity = match list.iter().take(3).map(|x| x.abs() % 2).sum::<i32>() < 2 {
//         true => 0,
//         false => 1,
//     };

//     *list.iter().find(|x| x.abs() % 2 != parity).unwrap()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let t1 = [2, 6, 8, -10, 3];
        let t2 = [
            206847684, 1056521, 7, 17, 1901, 21104421, 7, 1, 35521, 1, 7781,
        ];
        let t3 = [std::i32::MAX, 0, 1];
        assert_eq!(3, find_outlier(&t1));
        assert_eq!(206847684, find_outlier(&t2));
        assert_eq!(0, find_outlier(&t3));
    }
}
