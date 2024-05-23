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

    let mut r = 0;

    for l in 0..n {
        while r < n - 1 && a_list[r + 1] - a_list[l] <= k {
            r += 1;
        }

        ans += r - l;
    }

    print!("{}", ans);
}
