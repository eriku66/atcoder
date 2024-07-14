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
        q: usize,
        s: String,
        abcd_list: [(Usize1, Usize1, Usize1, Usize1); q],
    }

    for (a, b, c, d) in abcd_list {
        println!("{}", if s[a..=b] == s[c..=d] { "Yes" } else { "No" });
    }
}
