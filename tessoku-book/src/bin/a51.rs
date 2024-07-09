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
        q: usize,
    }

    let mut stack = Vec::new();

    for _ in 0..q {
        input! {t: usize}

        match t {
            1 => {
                input! {x: String}
                stack.push(x);
            }
            2 => {
                println!("{}", stack.last().unwrap());
            }
            3 => {
                stack.pop();
            }
            _ => {}
        }
    }
}
