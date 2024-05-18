#[allow(unused_imports)]
use itertools::{iproduct, Itertools};
use num::Integer;
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
        mut sc_list: [(String, usize); n],
    }

    sc_list.sort();

    let i = sc_list.iter().map(|(_, c)| c).sum::<usize>().mod_floor(&n);

    print!("{}", sc_list.get(i).unwrap().0);
}
