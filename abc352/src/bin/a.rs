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
        _n: usize,
        x: usize,
        y: usize,
        z: usize,
    }

    print!(
        "{}",
        if (x > y && x > z && z > y) || (x < y && x < z && z < y) {
            "Yes"
        } else {
            "No"
        }
    );
}
