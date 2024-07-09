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

    let mut queue = VecDeque::new();

    for _ in 0..q {
        input! {t: usize}

        match t {
            1 => {
                input! {x: String}
                queue.push_back(x);
            }
            2 => {
                println!("{}", queue.front().unwrap());
            }
            3 => {
                queue.pop_front();
            }
            _ => {}
        }
    }
}
