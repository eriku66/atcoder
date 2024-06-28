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
        n: usize,
        mut lr_list: [(usize, usize); n],
    }

    lr_list.sort_by(|a, b| a.1.cmp(&b.1));

    let mut ans = 0;
    let mut max = 0;

    for (l, r) in lr_list {
        if max <= l {
            max = r;
            ans += 1;
        }
    }

    print!("{}", ans);
}
