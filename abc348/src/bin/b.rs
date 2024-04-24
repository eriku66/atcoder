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
        points: [(isize, isize); n],
    }

    for (x1, y1) in &points {
        println!(
            "{}",
            points
                .iter()
                .enumerate()
                .max_by_key(|&(i, &(x2, y2))| (
                    x1.abs_diff(x2).pow(2) + y1.abs_diff(y2).pow(2),
                    -(i as isize)
                ))
                .unwrap()
                .0
                + 1
        );
    }
}
