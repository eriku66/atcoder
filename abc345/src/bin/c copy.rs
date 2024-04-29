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
        s: String,
    }

    let mut result = 0;
    let mut counts = [0usize; 26];

    for (j, c) in s.bytes().enumerate() {
        result += j - counts[(c - b'a') as usize];
        counts[(c - b'a') as usize] += 1;
    }

    if *counts.iter().max().unwrap() > 1 {
        result += 1;
    }

    print!("{}", result);
}
