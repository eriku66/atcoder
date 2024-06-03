#[allow(unused_imports)]
use itertools::{iproduct, Itertools};
#[allow(unused_imports)]
use num_traits::pow;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::iter::FromIterator;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: i64,
        mut a_list: [i64; n],
    }

    a_list.sort_unstable();

    if k >= 1 {
        println!("Yes");
        println!("{}", a_list.iter().join(" "));
        return;
    }

    let mut a_sum = vec![0; n + 1];

    for (i, &a) in a_list.iter().rev().enumerate() {
        a_sum[i + 1] = a_sum[i] + a;
    }

    if a_sum.iter().all(|&a| a >= k) {
        println!("Yes");
        println!("{}", a_list.iter().rev().join(" "));
        return;
    }

    print!("No");
}
