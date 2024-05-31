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
        h_list: [usize; n],
    }

    let mut dp = vec![0; n + 1];
    dp[2] = h_list[0].abs_diff(h_list[1]);

    for i in 3..=n {
        dp[i] = min(
            dp[i - 1] + h_list[i - 2].abs_diff(h_list[i - 1]),
            dp[i - 2] + h_list[i - 3].abs_diff(h_list[i - 1]),
        );
    }

    let mut ans = Vec::with_capacity(n);
    let mut i = n;

    loop {
        ans.push(i);

        if i == 1 { break; }

        if dp[i] - dp[i - 1] == h_list[i - 1].abs_diff(h_list[i - 2]) {
            i -= 1;
        } else {
            i -= 2;
        }
    }

    println!("{}", ans.len());
    println!("{}", ans.iter().rev().join(" "));
}
