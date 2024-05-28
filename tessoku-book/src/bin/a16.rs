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
        a_list: [usize; n - 1],
        b_list: [usize; n - 2],
    }

    let mut dp = vec![0; n];
    dp[1] = a_list[0];

    for i in 2..n {
        dp[i] = min(dp[i - 1] + a_list[i - 1], dp[i - 2] + b_list[i - 2]);
    }

    print!("{}", dp.last().unwrap());
}
