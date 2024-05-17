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
        h: usize,
        w: usize,
    }

    let mut grid = vec![vec![0; w]; h];

    for row in grid.iter_mut() {
        let mut total = 0;

        for c in row {
            input! {s: usize}
            total += s;

            *c = total;
        }
    }

    for i in 1..h {
        for j in 0..w {
            grid[i][j] += grid[i - 1][j];
        }
    }

    input! {
        q: usize,
    }

    for _ in 0..q {
        input! {
            a: usize,
            b: usize,
            c: usize,
            d: usize,
        }

        println!(
            "{}",
            grid[c - 1][d - 1]
                + if a >= 2 && b >= 2 {
                    grid[a - 2][b - 2]
                } else {
                    0
                }
                - if a >= 2 { grid[a - 2][d - 1] } else { 0 }
                - if b >= 2 { grid[c - 1][b - 2] } else { 0 }
        );
    }
}
