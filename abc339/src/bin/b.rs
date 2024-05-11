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
    }

    let moves = ['U', 'R', 'D', 'L'];
    let mut now_move = 0;
    let mut now_p = (0, 0);
    let mut grid = vec![vec!['.'; w]; h];

    for _ in 0..n {
        match grid[now_p.1][now_p.0] {
            '.' => {
                grid[now_p.1][now_p.0] = '#';

                now_move = (now_move + 1) % moves.len();
            }
            '#' => {
                grid[now_p.1][now_p.0] = '.';

                now_move = (now_move + moves.len() - 1) % moves.len();
            }
            _ => {}
        };

        match moves[now_move] {
            'U' => now_p.1 = (now_p.1 + h - 1) % h,
            'R' => now_p.0 = (now_p.0 + 1) % w,
            'D' => now_p.1 = (now_p.1 + 1) % h,
            'L' => now_p.0 = (now_p.0 + w - 1) % w,
            _ => {}
        }
    }

    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}
