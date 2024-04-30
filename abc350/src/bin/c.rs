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
    }

    let mut swap_list: Vec<(usize, usize)> = Vec::with_capacity(n);

    for i in 0..n {
        let mut a = a_list[i];

        while i != a - 1 {
            a_list.swap(i, a - 1);
            swap_list.push((i, a - 1));
            a = a_list[i];
        }
    }

    println!("{}", swap_list.len());

    for (i, j) in swap_list {
        println!("{} {}", i + 1, j + 1);
    }
}
