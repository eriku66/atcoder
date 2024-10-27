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
        c: usize,
        t_list: [usize; n],
    }

    let mut ans = 1;
    let mut pre_push_t = *t_list.first().unwrap();

    for t in t_list.into_iter().skip(0) {
        if t - pre_push_t >= c {
            ans += 1;
            pre_push_t = t;
        }
    }

    print!("{}", ans);
}
