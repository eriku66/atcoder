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
        n: u32,
    }

    let len = 3usize.pow(n);

    let mut ans = vec![vec!['#'; len]; len];

    for c in 1..=n as usize {
        let w_len = 3usize.pow((c as u32) - 1);
        let step = w_len * 2 + w_len;

        for i in (0..len).skip(w_len).step_by(step) {
            for j in (0..len).skip(w_len).step_by(step) {
                for i1 in i..i + w_len {
                    for j1 in j..j + w_len {
                        ans[i1][j1] = '.';
                    }
                }
            }
        }
    }

    for row in ans {
        println!("{}", row.iter().collect::<String>());
    }
}
