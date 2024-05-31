#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::iter::FromIterator;

#[allow(unused_imports)]
use itertools::{iproduct, Itertools};
#[allow(unused_imports)]
use num_traits::pow;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

#[fastout]
fn main() {
    input! {
        n: usize,
        a_list: [usize; n - 1],
        b_list: [usize; n - 2],
    }

    let mut dp = vec![0; n + 1];
    dp[2] = a_list[0];

    for i in 3..=n {
        dp[i] = min(dp[i - 1] + a_list[i - 2], dp[i - 2] + b_list[i - 3]);
    }

    let mut ans = Vec::with_capacity(n);

    let mut i = n;

    while i > 1 {
        ans.push(i);

        if dp[i] - dp[i - 1] == a_list[i - 2] {
            i -= 1;
        } else {
            i -= 2;
        }
    }

    ans.push(1);

    println!("{}", ans.len());
    println!("{}", ans.iter().rev().join(" "));
}
