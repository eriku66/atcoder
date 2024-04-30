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
        m: usize,
        b_list: [usize; m],
        l: usize,
        c_list: [usize; l],
        q: usize,
        x_list: [usize; q],
    }

    let mut sum_list: HashSet<usize> = HashSet::new();

    for a in &a_list {
        for b in &b_list {
            for c in &c_list {
                sum_list.insert(a + b + c);
            }
        }
    }

    for x in x_list {
        println!("{}", if sum_list.contains(&x) { "Yes" } else { "No" });
    }
}
