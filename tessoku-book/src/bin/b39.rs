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

#[fastout]
fn main() {
    input! {
        n: usize,
        d: usize,
        mut xy_list: [(Usize1, usize); n],
    }

    xy_list.sort_unstable_by_key(|&(x, _)| x);

    let mut queue = BinaryHeap::with_capacity(n);

    let mut xy_i = 0;

    let mut ans = 0;

    for i in 0..d {
        while xy_i < n && xy_list[xy_i].0 <= i {
            queue.push(xy_list[xy_i].1);
            xy_i += 1;
        }

        ans += queue.pop().unwrap_or(0);
    }

    print!("{}", ans);
}
