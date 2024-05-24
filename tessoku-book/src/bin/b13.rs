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
        k: usize,
        a_list: [usize; n],
    }

    let mut ans = 0;

    let mut total_list = Vec::with_capacity(n + 1);
    total_list.push(0);

    for i in 0..n {
        total_list.push(total_list[i] + a_list[i]);
    }

    let mut r = 0;

    for l in 0..n - 1 {
        while r < n && total_list[r + 1] - total_list[l] <= k {
            r += 1;
        }

        ans += r - l;
    }

    if *a_list.last().unwrap() < k {
        ans += 1
    };

    print!("{}", ans);
}
