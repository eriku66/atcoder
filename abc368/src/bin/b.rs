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
        mut a_list: [usize; n],
    }

    let mut zero_count = a_list.iter().filter(|&&a| a == 0).count();
    let mut ans = 0;

    while zero_count < n - 1 {
        a_list.sort_unstable_by(|a, b| b.cmp(a));

        if a_list[0] > 0 {
            a_list[0] -= 1;

            if a_list[0] == 0 {
                zero_count += 1;
            }
        }

        if a_list[1] > 0 {
            a_list[1] -= 1;

            if a_list[1] == 0 {
                zero_count += 1;
            }
        }

        ans += 1;
    }

    print!("{}", ans);
}
