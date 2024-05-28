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
        h_list: [usize; n],
    }

    let mut dp = vec![0; n];
    dp[1] = h_list[0].abs_diff(h_list[1]);

    for i in 2..n {
        dp[i] = min(
            dp[i - 1] + h_list[i - 1].abs_diff(h_list[i]),
            dp[i - 2] + h_list[i - 2].abs_diff(h_list[i]),
        );
    }

    print!("{}", dp.last().unwrap());
}
