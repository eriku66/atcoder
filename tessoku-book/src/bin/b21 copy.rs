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
        s: Chars,
    }

    let mut dp = vec![vec![0; n]; n];

    for (i, row) in dp.iter_mut().enumerate() {
        row[i] = 1;
    }

    for (i, w) in s.windows(2).enumerate() {
        if w[0] == w[1] {
            dp[i][i + 1] = 2;
        }
    }

    for l in (0..(n - 1)).rev() {
        for r in (l + 1)..n {
            dp[l][r] = dp[l + 1][r].max(dp[l][r - 1]);

            if s[l] == s[r] {
                dp[l][r] = dp[l][r].max(dp[l + 1][r - 1] + 2);
            }
        }
    }

    print!("{}", dp.first().unwrap().last().unwrap());
}
