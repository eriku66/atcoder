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

#[fastout]
fn main() {
    input! {
        sx: usize,
        sy: usize,
        tx: usize,
        ty: usize,
    }

    let mut ans = 0;

    let mut x_diff = sx.abs_diff(tx);

    if x_diff > 0 && sy % 2 == 1 && (sx % 2 == 0 && sx < tx || sx % 2 == 1 && sx > tx) {
        x_diff += 1;
    }

    if x_diff > 0 && sy % 2 == 0 && (sx % 2 == 1 && sx < tx || sx % 2 == 0 && sx > tx) {
        x_diff += 1;
    }

    let y_diff = sy.abs_diff(ty);

    if x_diff >= y_diff {
        ans += y_diff;
        x_diff -= y_diff;

        ans += x_diff / 2;
    } else {
        ans += y_diff;
    }

    print!("{}", ans);
}
