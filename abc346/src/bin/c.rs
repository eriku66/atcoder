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
        k: usize,
        a_list: [usize; n],
    }

    let mut a_set = HashSet::with_capacity(n);

    for a in a_list {
        if a <= k {
            a_set.insert(a);
        }
    }

    print!("{}", (1 + k) * k / 2 - a_set.iter().sum::<usize>());
}
