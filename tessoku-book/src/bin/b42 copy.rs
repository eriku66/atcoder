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
        ab_list: [(i64, i64); n],
    }

    let mut apbp = 0;
    let mut apbm = 0;
    let mut ambp = 0;
    let mut ambm = 0;

    for (a, b) in ab_list {
        apbp += (a + b).max(0);
        apbm += (a - b).max(0);
        ambp += (-a + b).max(0);
        ambm += (-a - b).max(0);
    }

    print!("{}", apbp.max(apbm).max(ambp).max(ambm));
}
