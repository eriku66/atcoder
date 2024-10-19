#[allow(unused_imports)]
use itertools::{iproduct, Itertools};
#[allow(unused_imports)]
use num_traits::pow;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Ordering;
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
        mut s: Chars,
        t: Chars,
    }

    let mut x = Vec::new();

    let o_i_list = s
        .iter()
        .zip(t.iter())
        .enumerate()
        .map(|(i, (c, h))| (i, c.cmp(&h)))
        .collect_vec();

    for &(i, o) in o_i_list.iter() {
        if o == Ordering::Greater {
            s[i] = t[i];

            x.push(s.iter().join(""));
        }
    }

    for &(i, o) in o_i_list.iter().rev() {
        if o == Ordering::Less {
            s[i] = t[i];

            x.push(s.iter().join(""));
        }
    }

    println!("{}", x.len());

    for e in x {
        println!("{}", e);
    }
}
