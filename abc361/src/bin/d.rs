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
        mut s: Chars,
        t: Chars,
    }

    if s.iter().filter(|&&c| c == 'W').count() != t.iter().filter(|&&c| c == 'W').count() {
        print!("-1");
        return;
    }

    s.extend("..".chars());

    loop {}
}
