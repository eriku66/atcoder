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
        a_list: [Usize1; n],
        w_list: [usize; n],
    }

    let mut l = Vec::with_capacity(n);

    for (&a, w) in a_list.iter().zip(w_list) {
        l.push((a, w));
    }

    l.sort_by_key(|(_, w)| *w);

    let mut boxes = vec![false; n];

    let mut used = vec![false; n];

    for (i, &(a, _)) in l.iter().enumerate().rev() {
        if !boxes[a] {
            boxes[a] = true;
            used[i] = true;
        }
    }

    print!(
        "{}",
        l.iter()
            .enumerate()
            .filter(|(i, (_, _))| !used[*i])
            .map(|(_, (_, w))| w)
            .sum::<usize>()
    )
}
