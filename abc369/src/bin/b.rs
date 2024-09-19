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
        as_list: [(isize, char); n],
    }

    let mut l = -1;
    let mut r = -1;
    let mut ans = 0;

    for (a, s) in as_list {
        match s {
            'L' => {
                if l != -1 {
                    ans += a.abs_diff(l);
                }

                l = a;
            }
            'R' => {
                if r != -1 {
                    ans += a.abs_diff(r);
                }

                r = a;
            }
            _ => {}
        }
    }

    print!("{}", ans);
}
