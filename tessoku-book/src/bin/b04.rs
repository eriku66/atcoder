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
        n: Chars,
    }

    print!(
        "{}",
        n.iter()
            .rev()
            .enumerate()
            .map(|(i, &c)| (c.to_digit(10).unwrap() * 2u32.pow(i as u32)))
            .sum::<u32>()
    );
}
