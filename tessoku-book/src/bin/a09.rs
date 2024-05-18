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
        for row in grid.iter_mut().skip(a).take(c - a + 1) {
            row[b] += 1;
            row[d + 1] -= 1;
        }
    }

    for row in grid.iter().skip(1).take(h) {
        let mut t = 0;
        println!(
            "{}",
            row.iter()
                .skip(1)
                .take(w)
                .map(|&c| {
                    t += c;
                    t
                })
                .join(" ")
        );
    }
}
