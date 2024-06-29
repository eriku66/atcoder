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
        s_list: Chars,
    }

    let mut l = vec![1; n];

    for (i, &s) in s_list.iter().enumerate() {
        if s == 'A' {
            l[i + 1] = l[i] + 1;
        }
    }

    for (i, &s) in s_list.iter().enumerate().rev() {
        if s == 'B' {
            l[i] = max(l[i], l[i + 1] + 1);
        }
    }

    print!("{}", l.iter().sum::<usize>());
}
