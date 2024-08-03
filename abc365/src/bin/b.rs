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
use std::collections::BinaryHeap;
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
        a_list: [usize; n],
    }

    let mut pq = BinaryHeap::with_capacity(n);

    for (i, &a) in a_list.iter().enumerate() {
        pq.push((a, i + 1));
    }

    pq.pop();

    print!("{}", pq.pop().unwrap().1);
}
