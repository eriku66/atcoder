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
        mut ac_list: [(usize, usize); n],
    }

    let l = ac_list
        .iter()
        .enumerate()
        .sorted_by(|(_, a), (_, c)| a.0.cmp(&c.0))
        .collect::<Vec<_>>();

    let mut ans = Vec::with_capacity(n);
    ans.push(l.last().unwrap().0);

    let mut front;
    front = *l.last().unwrap();

    for &current in l.iter().rev().skip(1) {
        if front.1 .1 > current.1 .1 {
            ans.push(current.0);
            front = current;
        }
    }

    ans.sort_unstable();

    println!("{}", ans.len());
    println!("{}", ans.iter().map(|&i| i + 1).join(" "));
}
