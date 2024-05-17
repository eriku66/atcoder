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

    for i in 0..h {
        for j in 0..w {
            input! {s: usize}

            grid[i][j] = if j == 0 { s } else { grid[i][j - 1] + s };
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

        let mut ans = 0;

        for row in grid.iter().skip(a - 1).take(c - a + 1) {
            ans += row[d - 1] - if b >= 2 { row[b - 2] } else { 0 };
        }

        println!("{}", ans);
    }
}
