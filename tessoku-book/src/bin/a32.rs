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
        a: usize,
        b: usize,
    }

    let mut l = vec![false; n + 1];

    for i in 1..=n {
        if i >= a {
            l[i] = !l[i - a];
        }

        if i >= b && !l[i] {
            l[i] = !l[i - b];
        }
    }

    if l[n] {
        print!("First");
    } else {
        print!("Second");
    }
}
