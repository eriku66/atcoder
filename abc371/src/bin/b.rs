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
        ab_list: [(Usize1, char); m],
    }

    let mut has_tarou = vec![false; n];

    for (a, b) in ab_list {
        if !has_tarou[a] && b == 'M' {
            has_tarou[a] = true;
            println!("Yes");
        } else {
            println!("No")
        }
    }
}
