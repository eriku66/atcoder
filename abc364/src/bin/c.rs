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
        mut a_list: [usize; n],
        mut b_list: [usize; n],
    }

    a_list.sort_unstable();
    b_list.sort_unstable();

    let mut a_total = 0;
    let mut a_count = 0;

    for (i, &a) in a_list.iter().rev().enumerate() {
        a_total += a;

        if a_total > x || i == n - 1 {
            a_count = i;
            break;
        }
    }

    let mut b_total = 0;
    let mut b_count = 0;

    for (i, &b) in b_list.iter().rev().enumerate() {
        b_total += b;

        if b_total > y || i == n - 1 {
            b_count = i;
            break;
        }
    }

    print!("{}", min(a_count, b_count) + 1);
}
