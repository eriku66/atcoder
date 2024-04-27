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
        grid_a: [Chars; n],
        grid_b: [Chars; n],
    }

    for (i, (row_a, row_b)) in grid_a.iter().zip(grid_b.iter()).enumerate() {
        for (j, (a, b)) in row_a.iter().zip(row_b.iter()).enumerate() {
            if a != b {
                print!("{} {}", i + 1, j + 1);
                return;
            }
        }
    }
}
