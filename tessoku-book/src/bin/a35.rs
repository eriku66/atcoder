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
        a_list: [usize; n],
    }

    let mut pyramid = vec![vec![]; n];
    pyramid[n - 1] = a_list;

    for i in (0..n - 1).rev() {
        pyramid[i] = pyramid[i + 1]
            .windows(2)
            .map(|w| match i % 2 {
                0 => max(w[0], w[1]),
                1 => min(w[0], w[1]),
                _ => 0,
            })
            .collect_vec();
    }

    print!("{}", pyramid[0][0]);
}
