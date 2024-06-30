#[allow(unused_imports)]
use itertools::{iproduct, Itertools};
use num::Integer;
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
        mut a_list: [usize; n],
    }

    a_list.sort_unstable();

    let mut total = 0;
    let mut ans = 0;

    for &a in a_list.iter().rev() {
        ans += total / a;
        total += a;
    }

    print!("{}", ans);
}
