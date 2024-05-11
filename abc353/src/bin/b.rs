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
        a_list: [usize; n],
    }

    let mut i = 0;
    let mut s = k;
    let mut count = 0;

    while i < a_list.len() {
        while s >= a_list[i] {
            s -= a_list[i];
            i += 1;

            if i == a_list.len() {
                break;
            }
        }

        s = k;
        count += 1;
    }

    print!("{}", count);
}
