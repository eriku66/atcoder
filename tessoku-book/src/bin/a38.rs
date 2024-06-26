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
        lrh_list: [(Usize1, Usize1, usize); n]
    }

    let mut days = vec![24; d];

    for &(l, r, h) in &lrh_list {
        for i in l..=r {
            days[i] = days[i].min(h);
        }
    }

    print!("{}", days.iter().sum::<usize>());
}
