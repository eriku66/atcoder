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
        s: usize,
        k: usize,
        p_q_list: [(usize, usize); n],
    }

    let mut total = 0;

    for (p, q) in p_q_list {
        total += p * q;
    }

    print!("{}", (if total >= s { 0 } else { k }) + total);
}
