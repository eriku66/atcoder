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

fn main() {
    input! {
        s: Chars,
        mut t: String,
    }

    let mut lower_t = t.to_lowercase().chars().collect_vec();

    if lower_t[2] == 'x' {
        lower_t.pop();
    }

    let len = lower_t.len();

    let mut i = 0;

    for c in s {
        if c == lower_t[i] {
            i += 1;

            if i == len {
                print!("Yes");
                return;
            }
        }
    }

    print!("No");
}
