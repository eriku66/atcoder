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
        q: usize,
    }

    let mut a_list = (1..=n).collect_vec();
    let mut is_order_reversed = false;

    for _ in 0..q {
        input! {query_no: usize}
        match query_no {
            1 => {
                input! {mut x: Usize1, y: usize}

                if is_order_reversed {
                    x = n - 1 - x;
                }

                a_list[x] = y;
            }
            2 => is_order_reversed = !is_order_reversed,
            3 => {
                input! {mut x: Usize1}

                if is_order_reversed {
                    x = n - 1 - x;
                }

                println!("{}", a_list[x]);
            }
            _ => {}
        }
    }
}
