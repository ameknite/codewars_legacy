fn main() {}

fn row_weights(array: Vec<u32>) -> (u32, u32) {
    (
        (0..array.len()).step_by(2).map(|x| array[x]).sum(),
        (1..array.len()).step_by(2).map(|x| array[x]).sum(),
    )
}

// fn row_weights(array: Vec<u32>) -> (u32, u32) {
//     (
//         array.iter().step_by(2).sum(),
//         array.iter().skip(1).step_by(2).sum()
//     )
// }

#[test]
fn basic_tests() {
    assert_eq!(row_weights(vec![13, 27, 49]), (62, 27));
    assert_eq!(row_weights(vec![50, 60, 70, 80]), (120, 140));
    assert_eq!(row_weights(vec![80]), (80, 0));
}
