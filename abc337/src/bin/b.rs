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
        s: Chars,
    }

    let mut pre_char = '.';
    let mut has_list = HashSet::with_capacity(3);

    for c in s {
        if pre_char != c {
            if pre_char as u8 > c as u8 || has_list.contains(&c) {
                print!("No");
                return;
            }

            pre_char = c;
            has_list.insert(c);
        }
    }

    print!("Yes");
}
