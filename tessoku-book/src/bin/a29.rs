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

fn power(a: u64, b: u32, m: u64) -> u64 {
    let mut p = a;
    let mut ans = 1;

    for i in 0..30 {
        if b / (1 << i) % 2 == 1 {
            ans *= p;
            ans %= m;
        }

        p = (p * p) % m;
    }

    ans
}

#[fastout]
fn main() {
    input! {
        a: u64,
        b: u32,
    }

    print!("{}", power(a, b, 1000000007));
}
