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

    let mut sorted_a_list = a_list
        .iter()
        .enumerate()
        .map(|(i, &a)| (a, i))
        .collect_vec();

    sorted_a_list.sort_unstable();

    let mut i = 0;
    let mut front = 0;
    let mut ans = vec![0; n];

    for (a, index) in sorted_a_list {
        if a != front {
            i += 1;
        }

        front = a;
        ans[index] = i;
    }

    print!("{}", ans.iter().join(" "));
}
