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

#[fastout]
fn main() {
    input! {
        y: usize,
    }

    print!(
        "{}",
        if y % 4 != 0 {
            365
        } else if y % 100 != 0 {
            366
        } else if y % 400 != 0 {
            365
        } else {
            366
        }
    );
}
