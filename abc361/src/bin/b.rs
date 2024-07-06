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
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
        f: usize,
        g: usize,
        h: usize,
        i: usize,
        j: usize,
        k: usize,
        l: usize,
    }

    if g < a && h < b && i < c && a < j && b < k && c < l
        || g < d && h < e && i < f && d < j && e < k && f < l
        || g > a && h > b && i > c && a > j && b > k && c > l
        || g > d && h > e && i > f && d > j && e > k && f > l
        || a < g && b < h && c < i && g < d && h < e && i < f
        || a < j && b < k && c < l && j < d && k < e && l < f
        || a > g && b > h && c > i && g > d && h > e && i > f
        || a > j && b > k && c > l && j > d && k > e && l > f
    {
        print!("Yes");
    } else {
        print!("No");
    }
}
