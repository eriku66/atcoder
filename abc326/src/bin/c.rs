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

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut a_list: [usize; n],
    }

    a_list.sort_unstable();

    let mut ans = 0;

    for i in 0..n {
        ans = max(ans, a_list.lower_bound(&(a_list[i] + m)) - i);
    }

    print!("{}", ans)
}
