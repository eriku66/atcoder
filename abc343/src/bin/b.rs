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
        a_square: [[u8; n]; n],
    }

    for a_row in a_square {
        println!(
            "{}",
            a_row
                .iter()
                .enumerate()
                .filter(|&(_, a)| *a == 1)
                .map(|(i, _)| i + 1)
                .join(" ")
        )
    }
}
