fn main() {
}

fn last<T: Clone>(slice: &[T]) -> T {
    slice.iter().next_back().unwrap().clone()
}

// fn last<T: Clone>(slice: &[T]) -> T {
//   let l = slice.last();
//   match l {
//     None => panic!("empty"),
//     Some(x) => x.clone(),
//   }
// }

#[test]
fn should_work_for_non_empty_list_of_integers() {
  assert_eq!(last(&[1, 2, 3]), 3);
}

#[test]
fn should_work_for_non_empty_list_of_strings() {
  assert_eq!(last(&["a", "b"]), "b");
}
