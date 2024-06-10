#[allow(unused_imports)]
use itertools::{iproduct, Itertools};
use num_integer::Roots;
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
    }

    let mut deleted = vec![false; n + 1];
    deleted[0] = true;
    deleted[1] = true;

    for d in 2..=n.sqrt() {
        deleted
            .iter_mut()
            .skip(d * 2)
            .step_by(d)
            .for_each(|b| *b = true);
    }

    for i in deleted
        .iter()
        .enumerate()
        .filter(|(_, &b)| !b)
        .map(|(i, _)| i)
    {
        println!("{}", i);
    }
}
