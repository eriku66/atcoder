#[allow(unused_imports)]
use itertools::{iproduct, Itertools};
#[allow(unused_imports)]
use num_traits::pow;
use proconio::marker::Bytes;
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
        s: Bytes,
        t: Bytes,
    }

    print!(
        "{}",
        if s[0].abs_diff(s[1]).min(5 - s[0].abs_diff(s[1]))
            == t[0].abs_diff(t[1]).min(5 - t[0].abs_diff(t[1]))
        {
            "Yes"
        } else {
            "No"
        }
    )
}
