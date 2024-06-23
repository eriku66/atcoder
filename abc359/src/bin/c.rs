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
        mut sx: usize,
        sy: usize,
        mut tx: usize,
        ty: usize,
    }

    if sx.abs_diff(sy) % 2 == 1 {
        sx -= 1;
    }

    if tx.abs_diff(ty) % 2 == 1 {
        tx -= 1;
    }

    let mut x_diff = sx.abs_diff(tx);

    let y_diff = sy.abs_diff(ty);

    let mut ans = y_diff;

    if x_diff >= y_diff {
        x_diff -= y_diff;

        ans += x_diff / 2;
    }

    print!("{}", ans);
}
