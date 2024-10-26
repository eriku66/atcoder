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
        mut xy_list: [(isize, isize); n],
    }

    xy_list.push((0, 0));

    let mut px = 0;
    let mut py = 0;
    let mut ans = 0.;

    for (x, y) in xy_list {
        ans += (((px - x).abs() * (px - x).abs() + (py - y).abs() * (py - y).abs()) as f64).sqrt();

        px = x;
        py = y;
    }

    print!("{}", ans);
}
