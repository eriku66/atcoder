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
        n: usize,
        m: usize,
        a: [usize; n],
    }

    if a.iter().sum::<usize>() <= m {
        println!("infinite");
        return;
    }

    let mut l = 0;
    let mut r = *a.iter().max().unwrap();

    while r - l > 1 {
        let mid = (l + r) / 2;
        if a.iter().map(|a| mid.min(*a)).sum::<usize>() <= m {
            l = mid;
        } else {
            r = mid;
        }
    }

    println!("{}", l);
}
