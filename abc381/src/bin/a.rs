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
        s: Chars,
    }

    if n % 2 == 1
        && s[0..n / 2].iter().all(|&c| c == '1')
        && s[n / 2] == '/'
        && s[n / 2 + 1..].iter().all(|&c| c == '2')
    {
        print!("Yes");
    } else {
        print!("No");
    }
}
