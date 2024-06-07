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

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];
    dp[0] = (0..=t.len()).collect_vec();

    for (i, row) in dp.iter_mut().enumerate() {
        row[0] = i;
    }

    for (i, &s_c) in s.iter().enumerate() {
        for (j, &t_c) in t.iter().enumerate() {
            dp[i + 1][j + 1] = (dp[i][j + 1] + 1)
                .min(dp[i + 1][j] + 1)
                .min(dp[i][j] + if s_c == t_c { 0 } else { 1 });
        }
    }

    print!("{}", dp[s.len()][t.len()]);
}
