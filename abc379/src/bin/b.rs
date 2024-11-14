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
        _n: usize,
        k: usize,
        s: Chars,
    }

    let mut ans = 0;
    let mut count = 0;

    for c in s {
        match c {
            'O' => {
                count += 1;
            }
            _ => {
                ans += count / k;

                count = 0;
            }
        }
    }

    ans += count / k;

    count = 0;

    print!("{}", ans);
}
