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
use std::collections::BTreeSet;
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::iter::FromIterator;

#[fastout]
fn main() {
    input! {
        q: usize,
    }

    let mut set = BTreeSet::new();

    for _ in 0..q {
        input! {t: usize}

        match t {
            1 => {
                input! {x: usize}

                set.insert(x);
            }
            2 => {
                input! {x: usize}

                set.remove(&x);
            }
            3 => {
                input! {x: usize}

                if let Some(&min) = set.range(x..).next() {
                    println!("{}", min);
                } else {
                    println!("-1");
                }
            }
            _ => {}
        }
    }
}
