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
        n: usize,
        q: usize,
        ht_list: [(char, Usize1); q],
    }

    let mut l = 0;
    let mut r = 1;

    let mut ope_count = 0;

    for (h, t) in ht_list.into_iter() {
        match h {
            'L' => {
                ope_count += match true {
                    _ if l < r && r < t => l + n - t,
                    _ if t < r && r < l => t + n - l,
                    _ => l.abs_diff(t),
                };

                l = t;
            }
            'R' => {
                ope_count += match true {
                    _ if r < l && l < t => r + n - t,
                    _ if t < l && l < r => t + n - r,
                    _ => r.abs_diff(t),
                };

                r = t;
            }
            _ => {}
        }
    }

    print!("{}", ope_count);
}
