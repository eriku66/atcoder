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
#[allow(unused_imports)]
use superslice::Ext;

// #[fastout]
fn main() {
    input! {
        n: usize,
        k: u32,
        r_list: [u32; n],
    }

    let n_u32 = n as u32;

    for a in "1".repeat(n).parse::<u32>().unwrap()..="5".repeat(n).parse::<u32>().unwrap() {
        let a_vec = (0..n_u32)
            .map(|i| a / 10_u32.pow(n_u32 - i - 1) % 10)
            .collect_vec();

        if a_vec
            .iter()
            .enumerate()
            .any(|(i, &v)| v == 0 || v > r_list[i])
        {
            continue;
        }

        if a_vec.iter().sum::<u32>() % k == 0 {
            println!(
                "{}",
                a_vec
                    .iter()
                    .filter_map(|&v| char::from_digit(v, 10))
                    .join(" ")
            );
        }
    }
}
