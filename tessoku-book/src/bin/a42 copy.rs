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
        k: usize,
        ab_list: [(usize, usize); n],
    }

    let mut max_count = 0;

    for a in 1..=100 {
        for b in 1..=100 {
            max_count = max_count.max(
                ab_list
                    .iter()
                    .filter(|&&(aa, bb)| a <= aa && aa <= a + k && b <= bb && bb <= b + k)
                    .count(),
            )
        }
    }

    print!("{}", max_count);
}
