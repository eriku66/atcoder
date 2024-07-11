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

    let mut y_d_map = vec![vec![]; d];

    for (x, y) in xy_list {
        y_d_map[x].push(y);
    }

    let mut queue = BinaryHeap::with_capacity(n);

    let mut ans = 0;

    for y_list in y_d_map {
        for y in y_list {
            queue.push(y);
        }

        ans += queue.pop().unwrap_or(0);
    }

    print!("{}", ans);
}
