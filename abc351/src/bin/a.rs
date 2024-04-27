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
        a_list: [isize; 9],
        b_list: [isize; 8],
    }

    let diff = a_list.iter().sum::<isize>() - b_list.iter().sum::<isize>();

    print!("{}", if diff < 0 { 0 } else { diff + 1 });
}
