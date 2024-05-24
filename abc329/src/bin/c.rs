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
        s: Chars,
    }

    let mut map = HashMap::with_capacity(26);

    let mut cnt = 1;

    for i in 0..n {
        if i == n - 1 || s[i] != s[i + 1] {
            let max = map.entry(s[i]).or_insert(0);
            *max = (*max).max(cnt);
            cnt = 1;
        } else {
            cnt += 1;
        }
    }

    print!("{}", map.values().sum::<usize>());
}
