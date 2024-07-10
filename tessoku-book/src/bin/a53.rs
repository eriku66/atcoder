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
use std::collections::BinaryHeap;
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::iter::FromIterator;

#[fastout]
fn main() {
    const MAX_X: usize = 1_000_000;

    input! {
        q: usize,
    }

    let mut priority_queue = BinaryHeap::with_capacity(q);

    for _ in 0..q {
        input! {t: usize}

        match t {
            1 => {
                input! {x: usize}

                priority_queue.push(MAX_X - x);
            }
            2 => println!("{}", MAX_X - priority_queue.peek().unwrap()),
            3 => {
                priority_queue.pop();
            }
            _ => {}
        }
    }
}
