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

fn main() {
    input! {
        s: Chars,
    }

    print!(
        "{}",
        s.iter()
            .enumerate()
            .map(|(i, &c)| (c, i))
            .sorted_by_key(|&(c, _)| c)
            .map(|(_, i)| i)
            .tuple_windows()
            .map(|(i, j)| i.abs_diff(j))
            .sum::<usize>()
    );
}
