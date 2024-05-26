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
        a_list: [Usize1; t],
    }

    let mut rows = vec![0; n];
    let mut cols = vec![0; n];
    let mut lines = vec![0; 2];

    for (index, &a) in a_list.iter().enumerate() {
        let (i, j) = (a / n, a % n);

        rows[i] += 1;
        cols[j] += 1;

        if i == j {
            lines[0] += 1;
        }

        if i + j == n - 1 {
            lines[1] += 1;
        }

        if rows[i] == n || cols[j] == n || lines[0] == n || lines[1] == n {
            print!("{}", index + 1);
            return;
        }
    }

    print!("-1");
}
