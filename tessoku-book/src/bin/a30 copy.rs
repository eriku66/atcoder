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

const M: u64 = 1_000_000_007;

fn mod_factorize(n: u64) -> u64 {
    if n > 1 {
        n * mod_factorize(n - 1) % M
    } else {
        1
    }
}

fn power(a: u64, b: u64) -> u64 {
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

    ans
}

#[fastout]
fn main() {
    input! {
        n: u64,
        r: u64,
    }

    let num = mod_factorize(n);
    let den = mod_factorize(r) * mod_factorize(n - r) % M;

    print!("{}", num * power(den, M - 2) % M);
}
