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
        a_list: [usize; n],
    }

    let mut l = vec![0; 100];

    for a in a_list {
        l[a % 100] += 1;
    }

    let mut ans = 0;

    for i in 1..50 {
        ans += l[i] * l[100 - i];
    }

    if l[0] >= 2 {
        ans += l[0] * (l[0] - 1) / 2;
    };

    if l[50] >= 2 {
        ans += l[50] * (l[50] - 1) / 2;
    };

    print!("{}", ans);
}
