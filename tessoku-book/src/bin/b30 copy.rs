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

const M: usize = 1_000_000_007;

fn mod_factorize(n: usize) -> usize {
    let mut ans = 1;

    for i in 1..=n {
        ans *= i;
        ans %= M;
    }

    ans
}

fn mod_power(a: usize, b: usize) -> usize {
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
        h: usize,
        w: usize,
    }

    let num = mod_factorize(h + w - 2);
    let den = mod_factorize(h - 1) * mod_factorize(w - 1) % M;

    print!("{}", num * mod_power(den, M - 2) % M);
}
