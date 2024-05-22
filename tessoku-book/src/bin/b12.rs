#[allow(unused_imports)]
use itertools::{iproduct, Itertools};
#[allow(unused_imports)]
use num_traits::pow;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Ordering;
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::iter::FromIterator;

#[fastout]
fn main() {
    input! {
        n: f64,
    }

    const MIN_DIFF: f64 = 0.001;

    let mut l = MIN_DIFF;
    let mut r = n / 2.0;

    while r - l > MIN_DIFF {
        let x = (l + r) / 2.0;

        match n.total_cmp(&(x.powf(3.0) + x)) {
            Ordering::Greater => l = x + MIN_DIFF,
            _ => r = x - MIN_DIFF,
        }
    }

    print!("{}", l);
}
