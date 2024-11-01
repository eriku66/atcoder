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
        s_list: [Chars; 8],
    }

    print!(
        "{}",
        s_list
            .iter()
            .filter(|s| s.iter().all(|&c| c == '.'))
            .count()
            * (0..8)
                .filter(|&i| s_list.iter().all(|s| s[i] == '.'))
                .count()
    );
}
