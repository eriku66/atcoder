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
        t: usize,
        a_list: [usize; t],
    }

    let mut grid = vec![vec![false; n]; n];

    for (d, &a) in a_list.iter().enumerate() {
        let i = (a - 1) / n;
        let j = (a - 1) % n;

        grid[i][j] = true;

        if grid[i].iter().all(|&c| c) {
            print!("{}", d + 1);
            return;
        }

        if grid.iter().all(|low| low[j]) {
            print!("{}", d + 1);
            return;
        }

        if i == j {
            if a - 1 == (n.pow(2) - 1) / 2
                && ((0..n).all(|x| grid[x][x]) || (0..n).all(|x| grid[x][n - x - 1]))
            {
                print!("{}", d + 1);
                return;
            } else if (0..n).all(|x| grid[x][x]) {
                print!("{}", d + 1);
                return;
            }
        } else if j == n - i - 1 && (0..n).all(|x| grid[x][n - x - 1]) {
            print!("{}", d + 1);
            return;
        }
    }

    print!("-1");
}
