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
        q: usize,
        mut a_list: [isize; n],
        bk_list: [(isize, usize); q],
    }

    a_list.sort_unstable();

    for (b, k) in bk_list {
        let mut l = 0;
        let mut r = n - k;

        while l < r {
            let mid = (l + r) / 2;

            if b - a_list[mid] < a_list[mid + k] - b {
                r = mid;
            } else {
                l = mid + 1;
            }
        }

        println!("{}", max(b - a_list[l], a_list[l + k - 1] - b));
    }
}
