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
        wx_list: [(usize, usize); n],
    }

    let mut ans = 0;

    for base_h in 0..24 {
        let mut total = 0;

        for &(w, x) in wx_list.iter() {
            let h = (base_h + x) % 24;

            if 9 <= h && h <= 17 {
                total += w;
            }
        }

        if total > ans {
            ans = total;
        }
    }

    print!("{}", ans);
}
