#[allow(unused_imports)]
use itertools::{iproduct, Itertools};
#[allow(unused_imports)]
use num_traits::pow;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Ordering;
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::iter::FromIterator;

#[fastout]
fn main() {
    input! {
        a: isize,
        m: isize,
        l: isize,
        r: isize,
    }

    let la = match l.cmp(&a) {
        Ordering::Equal => l,
        Ordering::Less => l + (a - l) % m,
        Ordering::Greater => {
            let diff_over = (l - a) % m;

            if diff_over == 0 {
                l
            } else {
                l - diff_over + m
            }
        }
    };

    let ra = match a.cmp(&r) {
        Ordering::Equal => r,
        Ordering::Less => r - (r - a) % m,
        Ordering::Greater => {
            let diff_over = (a - r) % m;

            if diff_over == 0 {
                r
            } else {
                r + diff_over - m
            }
        }
    };

    print!("{}", (ra - la) / m + 1);
}
