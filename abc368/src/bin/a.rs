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

// #[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a_list: [usize; n],
    }

    print!(
        "{}",
        a_list[n - k..]
            .iter()
            .chain(a_list[..n - k].iter())
            .join(" ")
    );
}
