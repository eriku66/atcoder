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
        n: Chars,
    }

    if n.iter().filter(|&&c| c == '1').count() == 1
        && n.iter().filter(|&&c| c == '2').count() == 2
        && n.iter().filter(|&&c| c == '3').count() == 3
    {
        print!("Yes");
    } else {
        print!("No");
    }
}
