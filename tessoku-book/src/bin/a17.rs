#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::iter::FromIterator;

#[allow(unused_imports)]
use itertools::{iproduct, Itertools};
#[allow(unused_imports)]
use num_traits::pow;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

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

    let mut ans = vec![dp.len()];

    let mut i = dp.len() - 1;

    while i >= 2 {
        if dp[i] - dp[i - 1] == a_list[i - 1] {
            ans.push(i);
            i -= 1;
        } else {
            ans.push(i - 1);
            i -= 2;
        }
    }

    if *ans.last().unwrap() != 1 {
        ans.push(1);
    }

    println!("{}", ans.len());
    println!("{}", ans.iter().rev().join(" "));
}
