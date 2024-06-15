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
        m: usize,
        s_list: [Chars; n],
    }

    let mut ans = 11;

    for b in 0..2usize.pow(n as u32) {
        let mut l = vec!['x'; m];
        let mut count = 0;

        for (_, s) in s_list
            .iter()
            .enumerate()
            .filter(|(i, _)| b / (1 << i) % 2 == 1)
        {
            count += 1;

            for (i, &c) in s.iter().enumerate() {
                if c == 'o' {
                    l[i] = 'o'
                }
            }
        }

        if l.iter().all(|&c| c == 'o') {
            ans = min(ans, count);
        }
    }

    print!("{}", ans);
}
