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
        mut a_list: [usize; n],
        q: usize,
        u_list: [usize; q],
    }

    a_list.sort_unstable();

    for u in u_list {
        println!(
            "{}",
            match a_list.binary_search(&u) {
                Ok(i) => i,
                Err(i) => i,
            }
        );
    }
}
