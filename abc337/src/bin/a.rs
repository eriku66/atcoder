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
        x_y_list: [(usize, usize); n],
    }

    let total = x_y_list
        .iter()
        .copied()
        .fold((0, 0), |total, (x, y)| (total.0 + x, total.1 + y));

    print!(
        "{}",
        match total.0.cmp(&total.1) {
            std::cmp::Ordering::Greater => "Takahashi",
            std::cmp::Ordering::Less => "Aoki",
            std::cmp::Ordering::Equal => "Draw",
        }
    )
}
