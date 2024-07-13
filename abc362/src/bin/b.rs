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
        ax: isize,
        ay: isize,
        bx: isize,
        by: isize,
        cx: isize,
        cy: isize,
    }

    let l1 = (ax - bx).pow(2) + (ay - by).pow(2);
    let l2 = (bx - cx).pow(2) + (by - cy).pow(2);
    let l3 = (ax - cx).pow(2) + (ay - cy).pow(2);

    if l1 == l2 + l3 || l2 == l1 + l3 || l3 == l1 + l2 {
        print!("Yes")
    } else {
        print!("No")
    }
}
