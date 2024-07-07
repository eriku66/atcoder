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

fn func(l1: usize, r1: usize, l2: usize, r2: usize) -> bool {
    !(r1 <= l2 || r2 <= l1)
}

#[fastout]
fn main() {
    input! {
        x1: usize,
        y1: usize,
        z1: usize,
        x2: usize,
        y2: usize,
        z2: usize,
        x3: usize,
        y3: usize,
        z3: usize,
        x4: usize,
        y4: usize,
        z4: usize,
    }

    if func(x1, x2, x3, x4) && func(y1, y2, y3, y4) && func(z1, z2, z3, z4) {
        print!("Yes");
    } else {
        print!("No");
    }
}
