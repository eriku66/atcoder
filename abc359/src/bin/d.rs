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
        _n: usize,
        k: usize,
        s: String,
    }

    if s.chars().collect_vec().windows(k).any(|w| {
        for i in 0..k / 2 {
            if w[i] != w[k - 1 - i] {
                return false;
            }
        }

        true
    }) {
        print!("0");
        return;
    }

    s.chars().filter(||)
}
