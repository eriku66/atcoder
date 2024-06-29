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
        d: usize,
        mut xy_list: [(Usize1, usize); n],
    }

    let mut used = vec![false; n];

    let mut ans = 0;

    for i in 0..d {
        if let Some((max_i, &(_, max_y))) = xy_list
            .iter()
            .enumerate()
            .filter(|&(index, &(x, _))| !used[index] && x <= i)
            .max_by_key(|(_, (_, y))| y)
        {
            used[max_i] = true;

            ans += max_y;
        }
    }

    print!("{}", ans);
}
