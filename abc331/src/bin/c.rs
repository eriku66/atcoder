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

    let mut sorted_a_list = a_list.clone();

    sorted_a_list.sort_unstable();
    sorted_a_list.reverse();

    let mut total = 0;
    let mut total_list = HashMap::with_capacity(n);

    for a in sorted_a_list {
        if total_list.get(&a).is_none() {
            total_list.insert(a, total);
        }

        total += a;
    }

    print!(
        "{}",
        a_list.iter().map(|a| total_list.get(a).unwrap()).join(" ")
    );
}
