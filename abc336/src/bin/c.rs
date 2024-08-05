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
        mut n: usize,
    }

    n -= 1;

    let mut ans = Vec::new();

    for i in (0..=((n as f64).ln() / 5f64.ln()).floor() as u32).rev() {
        let d = 5usize.pow(i);
        let v = n / d * d;
        ans.push((v / d) * 2);
        n -= v;
    }

    print!("{}", ans.iter().map(|v| v.to_string()).collect::<String>());
}
