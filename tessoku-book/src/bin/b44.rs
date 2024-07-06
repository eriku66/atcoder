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
        a_list: [[usize; n]; n],
        q: usize,
        queries: [(usize, Usize1, Usize1); q],
    }

    let mut i_list = (0..n).collect_vec();

    for (t, x, y) in queries {
        match t {
            1 => i_list.swap(x, y),
            2 => {
                println!("{}", a_list[i_list[x]][y]);
            }
            _ => {}
        }
    }
}
