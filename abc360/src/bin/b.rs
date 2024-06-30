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

fn main() {
    input! {
        s: Chars,
        t: String,
    }

    for size in 1..s.len() {
        for i in 0..size {
            if s.chunks(size)
                .filter(|w| w.len() > i)
                .map(|w| w[i])
                .collect::<String>()
                == t
            {
                print!("Yes");
                return;
            }
        }
    }

    print!("No");
}
