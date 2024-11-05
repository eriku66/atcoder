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
        qr_list: [(usize, usize); n],
        qc: usize,
        td_list: [(Usize1, usize); qc],
    }

    let calc_closest_date = |t: usize, d: usize| -> usize {
        let (q, r) = qr_list[t];
        let pre_max_date = d / q * q + r;

        if d <= pre_max_date {
            return pre_max_date;
        }

        return pre_max_date + q;
    };

    for (t, d) in td_list {
        println!("{}", calc_closest_date(t, d))
    }
}
