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
        b: usize,
    }

    let mut a: usize = 1;

    loop {
        match a.pow(a as u32).cmp(&b) {
            Ordering::Equal => {
                print!("{}", a);
                return;
            }
            Ordering::Greater => {
                break;
            }
            Ordering::Less => a += 1,
        }
    }

    print!("-1");
}
