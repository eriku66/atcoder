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
        pa_list: [(Usize1, usize); n],
    }

    let mut dp = vec![vec![0; n]; n];

    // dp[0][r]
    for r in (0..n - 1).rev() {
        dp[0][r] = dp[0][r + 1];

        if pa_list[r + 1].0 <= r {
            dp[0][r] += pa_list[r + 1].1;
        }
    }

    // dp[l][n - 1]
    for l in 1..n {
        dp[l][n - 1] = dp[l - 1][n - 1];

        if l <= pa_list[l - 1].0 {
            dp[l][n - 1] += pa_list[l - 1].1;
        }
    }

    for l in 1..n {
        for r in (l..n - 1).rev() {
            dp[l][r] = max(
                dp[l][r + 1]
                    + if l <= pa_list[r + 1].0 && pa_list[r + 1].0 <= r {
                        pa_list[r + 1].1
                    } else {
                        0
                    },
                dp[l - 1][r]
                    + if l <= pa_list[l - 1].0 && pa_list[l - 1].0 <= r {
                        pa_list[l - 1].1
                    } else {
                        0
                    },
            );
        }
    }

    print!("{}", (0..n).map(|i| dp[i][i]).max().unwrap());
}
