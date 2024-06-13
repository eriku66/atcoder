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
        a: u64,
        b: u32,
    }

    const D: u64 = 1000000007;

    let mut squares: Vec<u64> = vec![0; (b.ilog2() + 1) as usize];
    squares[0] = a;

    for i in 1..squares.len() {
        squares[i] = squares[i - 1] * squares[i - 1];

        if squares[i] > D {
            squares[i] %= D;
        }
    }

    let mut ans = 1;

    for (i, &s) in squares.iter().enumerate() {
        if b / 2u32.pow(i as u32) % 2 == 1 {
            ans *= s;

            if ans > D {
                ans %= D;
            }
        }
    }

    print!("{}", ans % D);
}
