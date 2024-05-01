#[allow(unused_imports)]
use itertools::{iproduct, Itertools};
use num_integer::Roots;
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
    }

    for i in (1..=n.cbrt()).rev() {
        let cube = i.pow(3);
        if is_palindrome(cube) {
            print!("{}", cube);
            return;
        }
    }
}

fn is_palindrome(n: usize) -> bool {
    let s = n.to_string().chars().collect::<Vec<char>>();
    let s_len = s.len();

    for i in 0..s_len / 2 {
        if s[i] != s[s_len - 1 - i] {
            return false;
        }
    }

    true
}
