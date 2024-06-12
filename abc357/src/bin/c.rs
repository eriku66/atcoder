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
        n: u32,
    }

    let mut v = vec!["#".to_string()];

    for _ in 0..n {
        let w = ".".repeat(v[0].len());
        let a = v.iter().map(|row| row.repeat(3));
        let b = v.iter().map(|row| format!("{}{}{}", row, w, row));
        v = a.clone().chain(b).chain(a).collect();
    }

    for row in v {
        println!("{}", row);
    }
}
