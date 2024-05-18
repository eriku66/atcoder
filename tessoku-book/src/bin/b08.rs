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
        xy_list: [(usize, usize); n],
        q: usize,
        abcd_list: [(usize, usize, usize, usize); q],
    }

    const LENGTH: usize = 1500;

    let mut grid = vec![vec![0; LENGTH + 1]; LENGTH + 1];

    for (x, y) in xy_list {
        grid[x][y] += 1;
    }

    for row in &mut grid {
        let mut t = 0;

        for c in row {
            t += *c;
            *c = t;
        }
    }

    for i in 1..=LENGTH {
        for j in 0..=LENGTH {
            grid[i][j] += grid[i - 1][j];
        }
    }

    for (a, b, c, d) in abcd_list {
        println!(
            "{}",
            grid[c][d] + grid[a - 1][b - 1] - grid[a - 1][d] - grid[c][b - 1]
        )
    }
}
