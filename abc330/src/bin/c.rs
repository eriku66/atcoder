#[allow(unused_imports)]
use itertools::{iproduct, Itertools};
use num_integer::Roots;
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
        d: usize,
    }

    let mut ans = usize::MAX;

    for x in 0..=d.sqrt() {
        let over = d - x.pow(2);
        let min_y = over.sqrt();
        ans = ans.min(min(over - min_y.pow(2), (min_y + 1).pow(2) - over));
    }

    print!("{}", ans);
}
