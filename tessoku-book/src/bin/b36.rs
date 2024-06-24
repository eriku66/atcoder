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
        k: usize,
        s: Chars,
    }

    let on_count = s.iter().filter(|&&c| c == '1').count();

    if n == on_count || (k.abs_diff(on_count) % 2 == 0) {
        print!("Yes");
        return;
    }

    print!("No");
}
