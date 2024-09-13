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

// #[fastout]
fn main() {
    input! {
        n: usize,
        mut h_list: [isize; n],
    }

    let mut t = 0;
    let mut i = 0;

    while i < n {
        let d = h_list[i] / 5;

        if d > 0 {
            t += d * 3;
            h_list[i] -= d * 5;
        } else {
            t += 1;
            h_list[i] -= if t % 3 == 0 { 3 } else { 1 };
        }

        if h_list[i] <= 0 {
            i += 1;
        }
    }

    print!("{}", t);
}
