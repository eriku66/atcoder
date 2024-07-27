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

// #[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        mut s: (Usize1, Usize1),
        grid: [Chars; h],
        mut x: Chars,
    }

    for c in x {
        match c {
            'L' => {
                if s.1 > 0 && grid[s.0][s.1 - 1] == '.' {
                    s.1 -= 1;
                }
            }
            'R' => {
                if s.1 + 1 < w && grid[s.0][s.1 + 1] == '.' {
                    s.1 += 1;
                }
            }
            'U' => {
                if s.0 > 0 && grid[s.0 - 1][s.1] == '.' {
                    s.0 -= 1;
                }
            }
            'D' => {
                if s.0 + 1 < h && grid[s.0 + 1][s.1] == '.' {
                    s.0 += 1;
                }
            }
            _ => {}
        }
    }

    print!("{} {}", s.0 + 1, s.1 + 1);
}
