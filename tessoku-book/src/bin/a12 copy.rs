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

    let mut l = 1;
    let mut r = 10usize.pow(9);

    while l < r {
        let m = (l + r) / 2;
        match k.cmp(&(a_list.iter().map(|a| m / a).sum::<usize>())) {
            Ordering::Greater => l = m + 1,
            _ => r = m,
        }
    }

    print!("{}", l);
}
