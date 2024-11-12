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
        _m: usize,
        s: Chars,
        t: Chars,
    }

    let is_prefix = t.starts_with(&s);
    let is_suffix = t.ends_with(&s);

    print!(
        "{}",
        if is_prefix && is_suffix {
            0
        } else if is_prefix {
            1
        } else if is_suffix {
            2
        } else {
            3
        }
    );
}
