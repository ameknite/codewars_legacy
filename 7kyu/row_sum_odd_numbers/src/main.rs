fn main() {}

fn row_sum_odd_numbers(n: i64) -> i64 {
    let total: i64 = (1..(1..=n).sum::<i64>() * 2).step_by(2).sum();
    let prev: i64 = (1..(1..=(n - 1)).sum::<i64>() * 2).step_by(2).sum();
    total - prev
}

// fn row_sum_odd_numbers(n: i64) -> i64 {
//     (1..)
//         .filter(|&x| x % 2 == 1) // From all odd numbers
//         .skip((0..n as usize).sum()) // Skip previous rows in the pyramid
//         .take(n as usize) // Take all the elements from that row
//         .sum() // And sum them
// }

// fn row_sum_odd_numbers(n:i64) -> i64 {
//   // beautiful math here. much easier to explain on board, but itsa kode!
//   // on Instruction we have five levels (possible for input n)
//   // let i be order no of element.
//   // let Ei be element itself. So now E1 = 1, E2 = 3, E3 = 5, E4 = 7 ... E7 = 13 ... E11 = 21 ...
//   // let Sn - target sum of n-th level.
//   // let (SE)t - sum(Ei, i = 1..t) (sum of our Ei elements where i goes from 1 to t)
//   // let Tn - index of the last element on level n. i.e. T3 = 6 (and E6 = 11), T4 = 10 (and E10 = 19)
//   // then Sn = (SE)Tn - (SE)T(n-1). It means that target sum is the sum of elements from first to last in n-th level
//   //    minus sum of elements from first to last in previous level.
//   // now we'll calculate (SE)x for any x.
//   // From school math we now sum(i, i = 1 .. n) = n(n+1)/2.
//   //    but our x is odd number, so (SE)x = sum(2*i - 1, i = 1 .. x)
//   //     = 2*sum(i, i = 1 .. x) - sum(1, i = 1 .. x) = 2 * x (x+1)/2 - x = x^2.
//   // in words it means that sum of first x odd numbers is x^2. It's pretty curious on its own and you
//   // could check it manually.
//   // Now we need to calculate Tn - index of the last element on level n.
//   // It's given, that we have 1 element on 1-st level, 2 elements on 2-nd level, 3 elements on 3-d level and so on.
//   // It means, that 2-nd level + 1-st level have 2 + 1 elemets, 3-d + 2-nd + 1-st have 3 + 2 + 1 and so on
//   // In math it means that n-th level with all previous levels have n*(n+1)/2 elements.
//   // so n-th level last element index is just a quantity of all elements on all level, so Tn = n * (n + 1)/2
//   //    and T(n-1) = (n - 1) * n / 2
//   // We have all the formulas: Sn = (SE)Tn - (SE)T(n-1) = (SE)(n * (n + 1) / 2) - (SE)((n - 1) * n / 2) =
//   //     = (n * (n + 1) / 2)^2 - ((n - 1) * n / 2)^2 = ... = n^3
//   //
//   //    the only thing that upsets me: (i64)^3 maybe much more that resulting i64.
//   n * n * n
// }

#[test]
fn returns_expected() {
    assert_eq!(row_sum_odd_numbers(1), 1);
    assert_eq!(row_sum_odd_numbers(42), 74088);
}
