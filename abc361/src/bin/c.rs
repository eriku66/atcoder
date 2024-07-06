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
        k: usize,
        mut a_list: [usize; n],
    }

    a_list.sort();

    let len = n - k - 1;
    let mut ans = usize::MAX;

    for i in 0..=k {
        ans = ans.min(a_list[len + i] - a_list[i]);
    }

    print!("{}", ans);
}
