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
        q: usize,
        x_list: [usize; q],
    }

    for x in x_list {
        if (2..=x.sqrt()).any(|d| x % d == 0) {
            println!("No");
        } else {
            println!("Yes");
        }
    }
}
