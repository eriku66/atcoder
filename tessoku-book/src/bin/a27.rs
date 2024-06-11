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
        mut a: usize,
        mut b: usize,
    }

    // print!("{}", gcd(a, b));

    for d in (1..a.min(b)).rev() {
        if a % d == 0 && b % d == 0 {
            println!("{}", d);
            break;
        }
    }
}
