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
    }

    let mut a_list = Vec::new();

    for i in 1..=n {
        input! {a: [Usize1; i]}

        a_list.push(a);
    }

    let mut j = 0;

    for i in 0..n {
        if i >= j {
            j = a_list[i][j];
        } else {
            j = a_list[j][i];
        }
    }

    print!("{}", j + 1);
}
