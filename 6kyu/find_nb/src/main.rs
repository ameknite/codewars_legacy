fn main() {
    println!("{}", find_nb(36));
}

fn find_nb(m: u64) -> i32 {
    let mut n: u64 = 1;
    let mut sum = 0;
    while sum < m {
        sum += n.pow(3);
        n += 1;
    }
    match sum == m {
        true => (n - 1) as i32,
        false => -1,
    }
}

// fn find_nb(n: u64) -> i32 {
//     let mut sum = 0_u64;
//     let l = (0_u64..)
//         .take_while(|&x| {
//             sum += x.pow(3);
//             sum < n
//         })
//         .count() as i32;
//     if sum == n {
//         l
//     } else {
//         -1
//     }
// }

// //  by formula computed with WolframAlpha:
// //      n = 1/2 (sqrt(8*sqrt(x) + 1) - 1)
// //  from formula of sum of n cubes:
// //      x = 1/2 ( (n(n+1)) / 2)^2

// fn find_nb(n: u64) -> i32 {
//     let mut n = n as f64;
//     n = (n.sqrt().mul_add(8., 1.).sqrt() - 1.) * 0.5;
//     if n.fract() > f64::EPSILON && n.fract() < 1_f64 - f64::EPSILON {
//         -1
//     } else {
//         n as i32
//     }

// }

fn testing(n: u64, exp: i32) -> () {
    assert_eq!(find_nb(n), exp);
}

#[test]
fn basics_find_nb() {
    testing(36, 3);
    testing(24723578342962, -1);
    testing(135440716410000, 4824);
    testing(40539911473216, 3568);
    testing(26825883955641, 3218);
}
