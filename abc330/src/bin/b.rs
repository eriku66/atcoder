#[allow(unused_imports)]
use itertools::{iproduct, Itertools};
use nalgebra::clamp;
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
        l: usize,
        r: usize,
        a_list: [usize; n],
    }

    print!("{}", a_list.iter().map(|&a| clamp(a, l, r)).join(" "));
}
