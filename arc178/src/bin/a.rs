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
        mut a_list: [usize; m],
    }

    a_list.sort_unstable();

    let mut l = (1..=n).collect::<Vec<_>>();

    for a in a_list {
        if [1, n].contains(&a) {
            print!("-1");
            return;
        }

        l.swap(a - 1, a);
    }

    print!("{}", l.iter().join(" "));
}
