#[allow(unused_imports)]
use itertools::{iproduct, Itertools};
#[allow(unused_imports)]
use num_traits::pow;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Ordering;
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

    let mut l = 0;
    let mut r = 10usize.pow(9);

    while l < r {
        let m = (r + l) / 2;

        match a_list.iter().map(|&a| m / a).sum::<usize>().cmp(&k) {
            Ordering::Less => l = m + 1,
            _ => r = m,
        }
    }

    print!("{}", l);
}
