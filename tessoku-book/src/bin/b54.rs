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

    let mut map = HashMap::with_capacity(n);

    for a in a_list {
        *map.entry(a).or_insert(0) += 1;
    }

    print!(
        "{}",
        map.iter()
            .map(|(_, &c)| if c > 1 { c * (c - 1) / 2 } else { 0 })
            .sum::<usize>()
    );
}
