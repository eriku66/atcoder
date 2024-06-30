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
        n: usize,
        h: usize,
        x_list: [usize; n],
    }

    let mut poles = x_list.clone();
    let mut is_fell_list = vec![false; n];
    poles.sort_unstable();

    let mut l = Vec::new();

    earthquake(&x_list, &mut is_fell_list, &poles, h, 'R', 0, &mut l);
    earthquake(&x_list, &mut is_fell_list, &poles, h, 'L', 0, &mut l);
}

fn earthquake(
    x_list: &[usize],
    is_fell_list: &mut [bool],
    poles: &[usize],
    h: usize,
    dir: char,
    i: usize,
    l: &mut [f64],
) -> f64 {
    match dir {
        'R' => h,
        'L' => {}
        _ => {}
    }

    if is_fell_list.iter().all(|b| *b) {
        1.0 / (i + 1) as f64
    } else {
        earthquake(x_list, is_fell_list, poles, h, 'R', i + 1, l)
            + earthquake(x_list, is_fell_list, poles, h, 'L', i + 1, l)
    }
}
