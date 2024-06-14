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
        a: u64,
        b: u64,
    }

    const M: u64 = 1000000007;

    let mut p = a;
    let mut ans = 1;

    for i in 0..=b.ilog2() {
        if b / (1 << i) % 2 == 1 {
            ans *= p;
            ans %= M;
        }

        p *= p;
        p %= M;
    }

    print!("{}", ans);
}
