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
        l: usize,
        ab_list: [(usize, char); n],
    }

    let mut ans = 0;

    for (a, b) in ab_list {
        match b {
            'E' => ans = ans.max(l - a),
            'W' => ans = ans.max(a),
            _ => {}
        }
    }

    print!("{}", ans);
}
