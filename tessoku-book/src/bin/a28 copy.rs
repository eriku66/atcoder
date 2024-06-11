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
        ta_list: [(char, isize); n],
    }

    let mut b = 0;

    for (t, a) in ta_list {
        match t {
            '+' => b += a,
            '-' => b -= a,
            '*' => b *= a,
            _ => {}
        }

        if b < 0 {
            b += 10000;
        }

        b %= 10000;

        println!("{}", b);
    }
}
