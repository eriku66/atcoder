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

fn main() {
    input! {
        mut m: usize,
    }

    let mut a_list = Vec::with_capacity(20);

    while m > 0 {
        let a = min(10, log_base_3(m));

        a_list.push(a);

        m -= 3_usize.pow(a as u32);
    }

    println!("{}", a_list.len());
    println!("{}", a_list.iter().join(" "));
}

fn log_base_3(m: usize) -> usize {
    ((m as f64).ln() / 3f64.ln()) as usize
}
