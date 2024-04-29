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
        mut w: usize,
        mut b: usize,
    }

    let n = w + b;

    let s = "wbwbwwbwbwbw".chars().collect::<Vec<char>>();

    let length = s.len();

    for start_i in 0..length {
        let mut w_count = 0;
        let mut b_count = 0;

        for i in 0..n {
            if s[(start_i + i) % length] == 'w' {
                w_count += 1;
            } else {
                b_count += 1;
            }
        }

        if w_count == w && b_count == b {
            print!("Yes");
            return;
        }
    }

    print!("No");
}
