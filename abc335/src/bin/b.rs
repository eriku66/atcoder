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
    }

    for x in 0..=n {
        for y in 0..=n {
            if x + y > n {
                break;
            }

            for z in 0..=n {
                if x + y + z > n {
                    break;
                }

                println!("{} {} {}", x, y, z);
            }
        }
    }
}
