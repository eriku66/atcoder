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
        p_list: [usize; n],
        q: usize,
        a_b_list: [(usize, usize); q],
    }

    let mut l = vec![0; n + 1];

    for (i, &p) in p_list.iter().enumerate() {
        l[p] = i;
    }

    for (a, b) in a_b_list {
        println!("{}", if l[a] < l[b] { a } else { b });
    }
}
