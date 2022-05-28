fn main() {}

fn gimme(input_array: [i32; 3]) -> usize {
    let mut sort_array = input_array;
    sort_array.sort();
    input_array.iter().position(|&x| x == sort_array[1]).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gimme() {
        assert_eq!(gimme([2, 3, 1]), 0);
        assert_eq!(gimme([-2, -3, -1]), 0);
        assert_eq!(gimme([5, 10, 14]), 1);
    }
}
