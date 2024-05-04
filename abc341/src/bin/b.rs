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
        mut a_list: [usize; n],
        s_t_list: [(usize, usize); n - 1]
    }

    for (i, &(s, t)) in s_t_list.iter().enumerate() {
        a_list[i + 1] += t * a_list[i].div_euclid(s);
    }

    print!("{}", a_list[n - 1]);
}
