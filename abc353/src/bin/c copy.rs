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
        mut a_list: [usize; n],
    }

    const M: usize = 10usize.pow(8);

    a_list.sort_unstable();

    let mut count = 0;
    let mut r = n;

    for i in 0..n {
        r = r.max(i + 1);

        while r - 1 > i && a_list[r - 1] + a_list[i] >= M {
            r -= 1;
        }

        count += n - r;
    }

    print!("{}", a_list.iter().sum::<usize>() * (n - 1) - count * M);
}
