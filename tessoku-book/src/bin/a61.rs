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
use std::collections::BTreeSet;
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::iter::FromIterator;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab_list: [(usize, usize); m],
    }

    let mut set = vec![BTreeSet::new(); n + 1];

    for (a, b) in ab_list {
        set[a].insert(b);
        set[b].insert(a);
    }

    for (i, points) in set.iter().enumerate().skip(1) {
        println!("{}: {:?}", i, points);
    }
}
