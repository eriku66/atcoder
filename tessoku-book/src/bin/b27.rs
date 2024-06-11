#[allow(unused_imports)]
use itertools::{iproduct, Itertools};
use num_integer::{gcd, lcm};
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
        a: usize,
        b: usize,
    }

    // print!("{}", lcm(a, b));

    print!("{}", a * b / gcd(a, b));
}
