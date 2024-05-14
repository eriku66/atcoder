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
        a_list: [usize; n],
        q: usize,
        l_q_list: [(usize, usize); q],
    }

    let mut total_list = Vec::with_capacity(n);
    total_list.push((0, 0));

    for (i, &a) in a_list.iter().enumerate() {
        let front = total_list[i];

        total_list.push(match a {
            1 => (front.0 + 1, front.1),
            0 => (front.0, front.1 + 1),
            _ => (0, 0),
        })
    }

    for (l, q) in l_q_list {
        match (total_list[q].0 - total_list[l - 1].0).cmp(&(total_list[q].1 - total_list[l - 1].1))
        {
            Ordering::Greater => println!("win"),
            Ordering::Less => println!("lose"),
            Ordering::Equal => println!("draw"),
        }
    }
}
