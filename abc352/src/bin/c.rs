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
        a_b_list: [(usize, usize); n],
    }

    let mut max_diff = 0;
    let mut total = 0;

    for (a, b) in a_b_list {
        let diff = b - a;

        if diff > max_diff {
            max_diff = diff;
        }

        total += a;
    }

    print!("{}", total + max_diff);
}
