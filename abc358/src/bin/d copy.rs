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
        m: usize,
        mut a_list: [usize; n],
        mut b_list: [usize; m],
    }

    a_list.sort_unstable();
    b_list.sort_unstable();

    let mut ans = 0;
    let mut i = 0;

    for b in b_list {
        while i < n && a_list[i] < b {
            i += 1;
        }

        if let Some(a) = a_list.get(i) {
            ans += a;
        } else {
            print!("-1");
            return;
        }

        i += 1;
    }

    print!("{}", ans);
}
