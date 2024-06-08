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
        mut m: usize,
        h_list: [usize; n],
    }

    let mut ans = 0;

    for h in h_list {
        if m >= h {
            m -= h;
            ans += 1;
        } else {
            break;
        }
    }

    print!("{}", ans);
}
