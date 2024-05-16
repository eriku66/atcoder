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

    let mut set = vec![0; d];

    for (l, r) in l_r_list {
        for c in set.iter_mut().skip(l - 1).take(r - l + 1) {
            *c += 1;
        }
    }

    for c in set {
        println!("{}", c)
    }
}
