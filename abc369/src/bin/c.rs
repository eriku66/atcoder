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
        a_list: [isize; n]
    }

    let mut ans = 0;
    let mut continue_count = 0;
    let mut pre_diff = -1;

    for (a, b) in a_list.iter().tuple_windows() {
        let diff = b - a;

        if diff == pre_diff {
            continue_count += 1;
        } else {
            continue_count = 1;
            pre_diff = diff;
        }

        ans += continue_count;
    }

    print!("{}", ans + n);
}
