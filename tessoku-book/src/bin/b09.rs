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
        abcd_list: [(usize, usize, usize, usize); n],
    }

    const LENGTH: usize = 1500;

    let mut grid = vec![vec![0; LENGTH + 2]; LENGTH + 2];

    for (a, b, c, d) in abcd_list {
        grid[a][b] += 1;
        grid[a][d] -= 1;
        grid[c][b] -= 1;
        grid[c][d] += 1;
    }

    let mut ans = 0;

    for j in 0..=LENGTH {
        if j > 0 {
            grid[0][j] += grid[0][j - 1];
        }

        if grid[0][j] > 0 {
            ans += 1;
        }
    }

    for i in 1..=LENGTH {
        let mut t = 0;

        for j in 0..=LENGTH {
            if grid[i][j] != 0 {
                t += grid[i][j];
            }

            grid[i][j] = grid[i - 1][j] + t;

            if grid[i][j] > 0 {
                ans += 1;
            }
        }
    }

    print!("{}", ans)
}
