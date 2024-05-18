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
        a_list: [usize; n],
        d: usize,
        lr_list: [(usize, usize); d],
    }

    let max_list = a_list.iter().skip(1).fold(vec![a_list[0]], |mut l, a| {
        l.push(*l.last().unwrap().max(a));
        l
    });

    let r_max_list = a_list
        .iter()
        .rev()
        .skip(1)
        .fold(vec![*a_list.last().unwrap()], |mut l, a| {
            l.push(*l.last().unwrap().max(a));
            l
        });

    for (l, r) in lr_list {
        println!("{}", max_list[l - 2].max(r_max_list[n - r - 1]));
    }
}
