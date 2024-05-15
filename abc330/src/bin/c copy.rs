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
use std::ops::Add;

#[fastout]
fn main() {
    input! {
        d: isize,
    }

    let mut min = 10isize.pow(12);

    for x in (0..).take_while(|x| x * x <= d) {
        let c = d - x.pow(2);
        let c_sqrt = c.sqrt();
        min = min.min(c - c_sqrt.pow(2));
        min = min.min(c_sqrt.add(1).pow(2) - c);
    }

    print!("{}", min);
}
