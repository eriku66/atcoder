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
        k_list: [usize; n],
    }

    let k_total = k_list.iter().sum::<usize>();
    let k_total_half = (k_total + 1) / 2;

    let mut ans = k_total;

    for i in (1..=n).rev() {
        for combination in k_list.iter().combinations(i) {
            let total = combination.into_iter().sum::<usize>();

            if total == k_total_half {
                print!("{}", total);
                return;
            }

            if total > k_total_half && total < ans {
                ans = total;
            }
        }
    }

    print!("{}", ans);
}
