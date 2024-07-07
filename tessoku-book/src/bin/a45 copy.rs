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
        _n: usize,
        c: char,
        a_list: Chars,
    }

    let value = |h: char| match h {
        'R' => 1,
        'B' => 2,
        _ => 0,
    };

    let sum = a_list.iter().map(|&c| value(c)).sum::<usize>();

    print!("{}", if value(c) == sum % 3 { "Yes" } else { "No" });
}
