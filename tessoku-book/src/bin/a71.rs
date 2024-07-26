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
        mut a_list: [usize; n],
        mut b_list: [usize; n],
    }

    a_list.sort_unstable();
    b_list.sort_unstable_by(|x, &y| y.cmp(x));

    print!(
        "{}",
        a_list
            .iter()
            .zip(b_list)
            .map(|(&a, b)| a * b)
            .sum::<usize>()
    );
}
