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
        d: usize,
        n: usize,
        l_r_list: [(usize, usize); n],
    }

    let mut diffs = vec![0; d + 1];

    for (l, r) in l_r_list {
        diffs[l - 1] += 1;
        diffs[r] -= 1;
    }

    let mut current = 0;

    for diff in diffs.iter().take(d) {
        current += diff;
        println!("{}", current);
    }
}
