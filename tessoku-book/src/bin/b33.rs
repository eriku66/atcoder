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
        _h: usize,
        _w: usize,
        ab_list: [(Usize1, Usize1); n],
    }

    if ab_list.iter().fold(0, |acc, &(a, b)| acc ^ a ^ b) == 0 {
        print!("Second");
    } else {
        print!("First");
    }
}
