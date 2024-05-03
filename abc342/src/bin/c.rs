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
        _: usize,
        mut s: String,
        q: usize,
        c_d_list: [(String, String); q],
    }

    let mut l = ('a'..='z').collect::<String>();

    for (c, d) in c_d_list.iter() {
        l = l.replace(c, d);
    }

    println!(
        "{}",
        s.bytes()
            .map(|b| l.chars().nth((b - b'a') as usize).unwrap())
            .collect::<String>()
    );
}
