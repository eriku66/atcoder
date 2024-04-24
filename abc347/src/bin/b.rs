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
        s: Chars
    }

    let length = s.len();

    let mut selected_strings: HashSet<String> = HashSet::new();

    for len in 1..=length {
        s.windows(len).for_each(|chars| {
            selected_strings.insert(chars.iter().collect());
        });
    }

    print!("{}", selected_strings.len());
}
