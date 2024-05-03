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
        _n: usize,
        mut s: Chars,
        q: usize,
        c_d_list: [(char, char); q],
    }

    let mut l: Vec<char> = ('a'..='z').collect();

    for (c, d) in c_d_list {
        for a in l.iter_mut() {
            if *a == c {
                *a = d;
            }
        }
    }

    println!(
        "{}",
        s.iter()
            .map(|&b| l[(b as u8 - b'a') as usize])
            .collect::<String>()
    );
}
