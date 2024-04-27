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
        n: usize,
        a_list: [usize; n]
    }

    let mut list: Vec<usize> = Vec::with_capacity(n);

    for a in a_list {
        list.push(a);

        loop {
            let length = list.len();

            if length <= 1 || list[length - 1] != list[length - 2] {
                break;
            }

            list.pop();
            *list.last_mut().unwrap() += 1;
        }
    }

    print!("{}", list.len());
}
