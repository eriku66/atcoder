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
        m: usize,
        ab_list: [(usize, usize); m],
    }

    let mut l = vec![HashSet::new(); n + 1];

    for (a, b) in ab_list {
        l[a].insert(b);
        l[b].insert(a);
    }

    print!(
        "{}",
        l.iter()
            .enumerate()
            .max_by_key(|(_, set)| set.len())
            .unwrap()
            .0
    );
}
