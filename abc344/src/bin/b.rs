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
use std::io::{self, BufRead};
#[allow(unused_imports)]
use std::iter::FromIterator;

#[fastout]
fn main() {
    let stdin = io::stdin();
    let mut a_list = stdin
        .lock()
        .lines()
        .filter_map(|l| l.unwrap().parse().ok())
        .collect::<Vec<usize>>();

    a_list.reverse();

    for a in a_list {
        println!("{}", a);
    }
}
