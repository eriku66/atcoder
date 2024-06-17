use ac_library::ModInt1000000007 as Mint;
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

fn fractorial_mod(n: usize) -> Mint {
    (1..=n).fold(Mint::new(1), |acc, i| acc * i)
}

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
    }

    let num = fractorial_mod(h + w - 2);
    let den = fractorial_mod(h - 1) * fractorial_mod(w - 1);

    print!("{}", num * den.inv());
}
