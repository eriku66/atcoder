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
        m: usize,
        a_list: [usize; m],
        x_list: [[usize; m]; n],
    }

    let mut x_sum = vec![0; m];

    for xs in x_list {
        for (i, &x) in xs.iter().enumerate() {
            x_sum[i] += x;
        }
    }

    for (i, &a) in a_list.iter().enumerate() {
        if a > x_sum[i] {
            print!("No");
            return;
        }
    }

    print!("Yes");
}
