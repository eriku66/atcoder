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
use std::usize;

#[fastout]
fn main() {
    input! {
        n: usize, q: usize,
        ope_ts: [usize; q],
    }

    let mut ts = vec![true; n];

    ope_ts
        .iter()
        .for_each(|ope_t| ts[ope_t - 1] = !ts[ope_t - 1]);

    print!("{}", ts.iter().filter(|&&t| t).count());
}
