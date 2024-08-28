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

#[fastout]
fn main() {
    input! {
        n: usize,
        mut s_list: [Chars; n],
    }

    let mut max = s_list[0].len();

    for s in s_list.iter_mut().skip(1) {
        match s.len().cmp(&max) {
            Ordering::Less => {
                s.append(&mut vec!['*'; max - s.len()]);
            }
            Ordering::Greater => {
                max = s.len();
            }
            _ => {}
        }
    }

    for i in 0..max {
        println!(
            "{}",
            s_list
                .iter()
                .rev()
                .filter_map(|s| s.get(i))
                .collect::<String>()
        );
    }
}
