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
        r: usize,
        g: usize,
        b: usize,
        c: String,
    }

    print!(
        "{}",
        match c.as_str() {
            "Red" => min(g, b),
            "Green" => min(r, b),
            "Blue" => min(r, g),
            _ => 0,
        }
    )
}
