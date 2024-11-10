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
        s_list: [Chars; n],
    }

    print!(
        "{}",
        s_list
            .iter()
            .enumerate()
            .sorted_by(|(_, a), (_, b)| {
                b.into_iter()
                    .filter(|&&c| c == 'o')
                    .count()
                    .cmp(&a.into_iter().filter(|&&c| c == 'o').count())
            })
            .map(|(i, _)| i + 1)
            .join(" ")
    );
}
