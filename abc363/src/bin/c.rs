#[allow(unused_imports)]
use itertools::{iproduct, Itertools};
#[allow(unused_imports)]
use num_traits::pow;
#[allow(unused_imports)]
use permutohedron::LexicalPermutation;
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
        _n: usize,
        k: usize,
        mut s: Chars,
    }

    s.sort_unstable();

    let mut ans = 0;

    loop {
        if !s
            .windows(k)
            .any(|w| (0..k / 2).all(|i| w[i] == w[k - i - 1]))
        {
            ans += 1;
        }

        if !s.next_permutation() {
            break;
        }
    }

    print!("{}", ans);
}
