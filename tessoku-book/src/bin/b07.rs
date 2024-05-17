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
        t: usize,
        n: usize,
        l_r_list: [(usize, usize); n],
    }

    let mut set = vec![0; t + 1];

    for (l, r) in l_r_list {
        set[l] += 1;
        set[r] -= 1;
    }

    let mut current = 0;

    for c in set.iter().take(t) {
        current += c;

        println!("{}", current);
    }
}
