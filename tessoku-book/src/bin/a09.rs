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
        n: usize,
        abcd_list: [(usize, usize, usize, usize); n],
    }

    let mut grid = vec![vec![0; w + 2]; h + 2];

    for (a, b, c, d) in abcd_list {
        grid[a][b] += 1;
        grid[a][d + 1] -= 1;
        grid[c + 1][b] -= 1;
        grid[c + 1][d + 1] += 1;
    }

    for i in 1..=h {
        let mut t = 0;

        for j in 1..=w {
            if grid[i][j] != 0 {
                t += grid[i][j];
            }

            grid[i][j] = grid[i - 1][j] + t;
        }
    }

    for row in grid.iter().skip(1).take(h) {
        println!("{}", row.iter().skip(1).take(w).join(" "));
    }
}
