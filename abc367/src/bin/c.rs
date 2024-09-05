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
#[allow(unused_imports)]
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: u32,
        r_list: [u32; n],
    }

    for a_list in r_list.into_iter().map(|r| 1..=r).multi_cartesian_product() {
        if a_list.iter().sum::<u32>() % k == 0 {
            println!("{}", a_list.into_iter().format(" "));
        }
    }
}
