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

    print!(
        "{}",
        if s.iter().enumerate().all(|(i, c)| {
            if i == 0 {
                c.is_ascii_uppercase()
            } else {
                c.is_ascii_lowercase()
            }
        }) {
            "Yes"
        } else {
            "No"
        }
    );
}
