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
        _x: usize,
        _y: usize,
        a_list: [usize; n],
    }

    fn find_grundy_number(a: usize) -> usize {
        match a % 5 {
            0 | 1 => 0,
            2 | 3 => 1,
            4 => 2,
            _ => 0,
        }
    }

    if a_list
        .iter()
        .map(|&a| find_grundy_number(a))
        .fold(0, |acc, ga| acc ^ ga)
        == 0
    {
        print!("Second");
    } else {
        print!("First");
    }
}
