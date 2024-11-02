#[allow(unused_imports)]
use itertools::{iproduct, Itertools};
#[allow(unused_imports)]
use num_traits::pow;
use proconio::marker::Isize1;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::TryInto;
#[allow(unused_imports)]
use std::iter::FromIterator;
#[allow(unused_imports)]
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab_list: [(Isize1, Isize1); m],
    }

    let mut unavailable_squares = HashSet::new();

    let mut check_square = |i: isize, j: isize| {
        if 0 <= i
            && i < n.try_into().unwrap()
            && 0 <= j
            && j < n.try_into().unwrap()
            && !unavailable_squares.contains(&(i, j))
        {
            unavailable_squares.insert((i, j));
        }
    };

    for (a, b) in ab_list {
        check_square(a, b);
        check_square(a + 2, b + 1);
        check_square(a + 1, b + 2);
        check_square(a - 1, b + 2);
        check_square(a - 2, b + 1);
        check_square(a - 2, b - 1);
        check_square(a - 1, b - 2);
        check_square(a + 1, b - 2);
        check_square(a + 2, b - 1);
    }

    print!(
        "{}",
        (n as u128) * (n as u128) - unavailable_squares.iter().count() as u128
    );
}
