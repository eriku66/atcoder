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
        max_weight: isize,
        wv_list: [(isize, isize); n],
    }

    const V_MAX: usize = 100000;

    let mut dp = vec![vec![-1; V_MAX + 1]; n + 1];
    dp[0][0] = 0;

    for (i, &(w, v)) in wv_list.iter().enumerate() {
        for j in 0..=V_MAX {
            if dp[i][j] == -1 {
                continue;
            }

            if dp[i + 1][j] == -1 {
                dp[i + 1][j] = dp[i][j];
            } else {
                dp[i + 1][j] = dp[i + 1][j].min(dp[i][j]);
            }

            if max_weight >= dp[i][j] + w {
                dp[i + 1][j + v as usize] = dp[i][j] + w;
            }
        }
    }

    print!("{:?}", dp[n].iter().rposition(|&c| c != -1).unwrap());
}
