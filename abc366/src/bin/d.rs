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
#[allow(unused_imports)]
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut n_square: [[[usize; n]; n]; n],
        q: usize,
        queries: [(Usize1, Usize1, Usize1, Usize1, Usize1, Usize1); q],
    }

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if k > 0 {
                    n_square[i][j][k] += n_square[i][j][k - 1];
                }
            }

            for k in 0..n {
                if i > 0 {
                    n_square[i][j][k] += n_square[i - 1][j][k];
                }
            }
        }
    }

    for (lx, rx, ly, ry, lz, rz) in queries {
        let mut sum = 0;

        for n_row in n_square[rx].iter().skip(ly).take(ry - ly + 1) {
            sum += n_row[rz];

            if lz > 0 {
                sum -= n_row[lz - 1];
            }
        }

        if lx > 0 {
            for n_row in n_square[lx - 1].iter().skip(ly).take(ry - ly + 1) {
                sum -= n_row[rz];

                if lz > 0 {
                    sum += n_row[lz - 1];
                }
            }
        }

        println!("{}", sum);
    }
}
