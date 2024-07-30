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
        ac_list: [(usize, usize); n],
    }

    let mut l = HashMap::with_capacity(n);

    for (a, c) in ac_list {
        let v = l.entry(c).or_insert(1_000_000_000);
        *v = min(*v, a);
    }

    print!("{}", l.values().max().unwrap());
}
