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
        q: usize,
        query_list: [(u8, usize); q],
    }

    let mut a = Vec::with_capacity(q);

    for (query_type, n) in query_list {
        match query_type {
            1 => a.push(n),
            2 => println!("{}", a[a.len() - n]),
            _ => {}
        }
    }
}
