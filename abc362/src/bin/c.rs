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
        lr_list: [(isize, isize); n],
    }

    let mut sum = lr_list.iter().map(|&(_, r)| r).sum::<isize>();

    if sum < 0 {
        print!("No");
        return;
    }

    let mut ans = Vec::new();

    for (l, r) in lr_list {
        if sum == 0 {
            ans.push(r);
        }

        if sum > 0 {
            let m = min(sum, l.abs_diff(r) as isize);

            ans.push(r - m);
            sum -= m;
        }
    }

    if sum > 0 {
        print!("No");
        return;
    }

    println!("Yes");
    print!("{}", ans.iter().join(" "));
}
