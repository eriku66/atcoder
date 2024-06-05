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
        max_weight: usize,
        wv_list: [(usize, usize); n],
    }

    let mut dp = vec![vec![0; max_weight + 1]; n + 1];

    for (i, &(w, v)) in wv_list.iter().enumerate() {
        let mut new_row = vec![0; max_weight + 1];

        for (j, &c) in dp[i].iter().enumerate() {
            new_row[j] = new_row[j].max(c);

            if max_weight >= j + w {
                new_row[j + w] = c + v;
            }
        }

        dp[i + 1] = new_row;
    }

    print!("{}", dp[n].iter().max().unwrap());
}
