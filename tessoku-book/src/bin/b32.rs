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
        a_list: [usize; k],
    }

    let mut l = vec![false; n + 1];

    for i in 1..=n {
        for &a in &a_list {
            if i >= a {
                l[i] = !l[i - a];
            }

            if l[i] {
                break;
            }
        }
    }

    if l[n] {
        print!("First");
    } else {
        print!("Second");
    }
}
