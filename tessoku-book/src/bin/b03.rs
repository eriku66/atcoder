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
        a_list: [usize; n],
    }

    let a_len = a_list.len();

    for i in 0..a_len {
        for ii in i + 1..a_len {
            for iii in ii + 1..a_len {
                if a_list[i] + a_list[ii] + a_list[iii] == 1000 {
                    print!("Yes");
                    return;
                }
            }
        }
    }

    print!("No");
}
