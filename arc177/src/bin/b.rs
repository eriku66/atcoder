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
        s_list: Chars,
    }

    let mut t_list = "".to_string();

    let mut front = '0';

    for (i, &s) in s_list.iter().rev().enumerate() {
        if s == front {
            continue;
        }

        match s {
            '0' => t_list.push_str(&"B".repeat(n - i).to_string()),
            '1' => t_list.push_str(&"A".repeat(n - i).to_string()),
            _ => {}
        }

        front = s;
    }

    println!("{}", t_list.len());
    println!("{}", t_list);
}
