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
        s: Chars,
        t: Chars,
    }

    let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];

    for (i, &sc) in s.iter().enumerate() {
        for (j, &tc) in t.iter().enumerate() {
            if sc == tc {
                dp[i + 1][j + 1] = (dp[i][j] + 1).max(dp[i][j + 1]).max(dp[i + 1][j]);
            } else {
                dp[i + 1][j + 1] = dp[i][j + 1].max(dp[i + 1][j]);
            }
        }
    }

    print!("{}", dp[s.len()][t.len()]);
}
