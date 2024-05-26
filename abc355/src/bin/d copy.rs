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

    let mut l_list = Vec::with_capacity(n);
    let mut r_list = Vec::with_capacity(n);

    for (l, r) in lr_list {
        l_list.push(l);
        r_list.push(r);
    }

    l_list.sort_unstable();
    r_list.sort_unstable();

    let mut ans = n * (n - 1) / 2;

    let mut j = 0;

    for i in 0..n {
        while r_list[j] < l_list[i] {
            j += 1;
        }

        ans -= j;
    }

    print!("{}", ans);
}
