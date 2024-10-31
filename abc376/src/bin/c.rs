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

fn main() {
    input! {
        n: usize,
        mut a_list: [usize; n],
        mut b_list: [usize; n - 1],
    }

    a_list.sort_unstable();
    b_list.sort_unstable();

    let mut x = 0;

    for a in a_list.into_iter().rev() {
        if b_list.len() == 0 {
            x = a;

            break;
        }

        let last_b = *b_list.last().unwrap();

        if a <= last_b {
            b_list.pop();

            continue;
        }

        if x != 0 {
            print!("-1");

            return;
        }

        x = a;
    }

    print!("{}", x);
}
