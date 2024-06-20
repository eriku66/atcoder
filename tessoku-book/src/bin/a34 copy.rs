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
        x: usize,
        y: usize,
        a_list: [usize; n],
    }

    let mut grundy = vec![0; 100_001];

    for i in 0..grundy.len() {
        grundy[i] = (0..=2)
            .filter(|&g| !(i >= x && g == grundy[i - x] || i >= y && g == grundy[i - y]))
            .min()
            .unwrap();
    }

    if a_list
        .iter()
        .map(|&a| grundy[a])
        .fold(0, |acc, ga| acc ^ ga)
        == 0
    {
        print!("Second");
    } else {
        print!("First");
    }
}
