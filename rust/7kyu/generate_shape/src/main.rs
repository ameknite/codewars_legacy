fn main() {}

fn generate_shape(n: i32) -> String {
    (0..n)
        .map(|_| "+".repeat(n as usize))
        .collect::<Vec<_>>()
        .join("\n")
}

// fn generate_shape(n: usize) -> String {
//     vec!["+".repeat(n); n].join("\n")
// }

#[test]
fn sample_test() {
    assert_eq!(generate_shape(3), "+++\n+++\n+++");
}
