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
    }

    let d = 1000000007;

    let mut two_previous = 1;
    let mut one_previous = 1;

    for _ in 0..n - 2 {
        let mut next = two_previous + one_previous;

        next %= d;

        two_previous = one_previous;
        one_previous = next;
    }

    println!("{}", one_previous);
}
