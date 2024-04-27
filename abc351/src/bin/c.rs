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

    let mut list: Vec<usize> = Vec::new();

    for a in a_list {
        list.push(a);

        loop {
            let length = list.len();

            if length <= 1 || list[length - 1] != list[length - 2] {
                break;
            }

            let sum = list[length - 1] + 1;

            list.pop();
            list.pop();
            list.push(sum);
        }
    }

    print!("{}", list.len());
}
