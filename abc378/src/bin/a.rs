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
        mut a_list: [usize; 4],
    }

    a_list.sort_unstable();

    print!(
        "{}",
        if a_list[0] == a_list[3] || (a_list[0] == a_list[1] && a_list[2] == a_list[3]) {
            2
        } else if a_list.windows(2).any(|w| w[0] == w[1]) {
            1
        } else {
            0
        }
    );
}
