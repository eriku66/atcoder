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
        q_list: [usize; n],
        a_list: [usize; n],
        b_list: [usize; n],
    }

    let mut ans = 0;

    for a_cnt in 0..=*q_list.iter().max().unwrap() {
        let b_cnt = q_list
            .iter()
            .enumerate()
            .filter_map(|(i, &q)| {
                if q < a_list[i] * a_cnt {
                    Some(0)
                } else if b_list[i] > 0 {
                    Some((q - a_list[i] * a_cnt) / b_list[i])
                } else {
                    None
                }
            })
            .min()
            .unwrap_or(0);

        if b_cnt > 0 {
            ans = max(ans, a_cnt + b_cnt)
        }
    }

    print!("{}", ans);
}
