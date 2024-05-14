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
        q: usize,
        a_list: [usize; n],
        l_q_list: [(usize, usize); q],
    }

    let mut total_list = Vec::with_capacity(n + 1);
    total_list.push(0);

    for (i, &a) in a_list.iter().enumerate() {
        total_list.push(total_list[i] + a);
    }

    for (l, q) in l_q_list {
        println!("{}", total_list[q] - total_list[l - 1]);
    }
}
