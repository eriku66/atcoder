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
        n: usize,
        x: usize,
        a_list: [usize; n],
    }

    let mut l = 0;
    let mut r = n;

    loop {
        let i = (r + l) / 2;

        match x.cmp(&a_list[i]) {
            Ordering::Equal => {
                println!("{}", i + 1);
                break;
            }
            Ordering::Greater => l = i,
            Ordering::Less => r = i,
        }
    }
}
