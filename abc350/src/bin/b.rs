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
use std::usize;

#[fastout]
fn main() {
    input! {
        n: usize, q: usize,
        ope_teeth: [usize; q],
    }

    let mut teeth_exists = vec![true; n + 1];

    teeth_exists[0] = false;

    for ope_tooth in ope_teeth {
        teeth_exists[ope_tooth] = !teeth_exists[ope_tooth];
    }

    print!("{}", teeth_exists.iter().filter(|&t| *t).count());
}
