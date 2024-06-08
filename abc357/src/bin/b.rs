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

    if s.iter().filter(|&c| c.is_uppercase()).count() > s.len() / 2 {
        print!(
            "{}",
            s.iter()
                .map(|&c| c.to_ascii_uppercase())
                .collect::<String>()
        );
    } else {
        print!(
            "{}",
            s.iter()
                .map(|&c| c.to_ascii_lowercase())
                .collect::<String>()
        );
    }
}
