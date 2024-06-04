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
        s: usize,
        a_list: [usize; n],
    }

    let mut dp = vec![vec![false; s + 1]; n + 1];
    dp[0][0] = true;

    for (i, &a) in a_list.iter().enumerate() {
        let mut new_row = vec![false; s + 1];

        for (j, _) in dp[i].iter().enumerate().filter(|(_, &c)| c) {
            new_row[j] = true;

            if s >= j + a {
                new_row[j + a] = true;
            }
        }

        dp[i + 1] = new_row;
    }

    if dp[n][s] {
        print!("Yes");
    } else {
        print!("No");
    }
}
