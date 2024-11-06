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
#[allow(unused_imports)]
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        a_list: [isize; n],
    }

    let mut last_appeared_map = HashMap::new();

    let mut b_list = Vec::with_capacity(n);

    for (i, a) in a_list.into_iter().enumerate() {
        if last_appeared_map.contains_key(&a) {
            b_list.push(*last_appeared_map.get(&a).unwrap() as isize + 1);
        } else {
            b_list.push(-1);
        }

        *last_appeared_map.entry(a).or_insert(0) = i;
    }

    print!("{}", b_list.iter().join(" "));
}
