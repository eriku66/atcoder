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
        month: usize,
        date: usize,
        mut y: usize,
        mut m: usize,
        mut d: usize,
    }

    if d < date {
        print!("{} {} {}", y, m, d + 1);
        return;
    }

    d = 1;

    if m < month {
        print!("{} {} {}", y, m + 1, d);
        return;
    }

    m = 1;

    print!("{} {} {}", y + 1, m, d);
}
