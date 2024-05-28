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
        q: usize,
        s: String,
        lr_list: [(Usize1, Usize1); q],
    }

    let mut cnt = 0;
    let mut front = '.';
    let mut sum_list = Vec::with_capacity(n);

    for c in s.chars() {
        if c == front {
            cnt += 1;
        }

        front = c;
        sum_list.push(cnt);
    }

    for (l, r) in lr_list {
        println!("{}", sum_list[r] - sum_list[l]);
    }
}
