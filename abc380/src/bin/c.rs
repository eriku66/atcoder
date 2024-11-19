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
#[allow(unused_imports)]
use superslice::Ext;

fn main() {
    input! {
        _n: usize,
        mut k: usize,
        s: Chars,
    }

    let mut pre_c = '.';
    let mut masses: Vec<String> = Vec::new();
    let mut swap_idx = 0;

    for c in s {
        if c == pre_c {
            masses.last_mut().unwrap().push(c);
        } else {
            masses.push(c.to_string());

            if k > 0 && c == '1' {
                k -= 1;

                if k == 0 {
                    swap_idx = masses.len() - 1;
                }
            }
        }

        pre_c = c;
    }
    masses.swap(swap_idx - 1, swap_idx);

    println!("{}", masses.join(""));
}
