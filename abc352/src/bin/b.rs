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
        mut s: Chars,
        t: Chars,
    }

    let mut si = 0;
    let mut l = Vec::with_capacity(s.len());

    for (i, c) in t.iter().enumerate() {
        if s[si] == *c {
            l.push(i + 1);
            si += 1;
        }
    }

    print!("{}", l.iter().join(" "));
}
