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
        s: Chars,
    }

    print!(
        "{}",
        if s.iter()
            .enumerate()
            .filter(|(i, _)| i % 2 == 1)
            .all(|(_, &c)| c == '0')
        {
            "Yes"
        } else {
            "No"
        }
    );
}