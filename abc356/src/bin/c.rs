use itertools::fold;
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
        k: usize,
    }

    let mut ar_list = Vec::with_capacity(m);

    for _ in 0..m {
        input! {
            c: usize,
            a_list: [Usize1; c],
            r: char,
        }

        ar_list.push((a_list, r));
    }

    let mut ans = 0;

    for b in 0..2usize.pow(n as u32) {
        let key_list = (0..n).map(|i| b / 2usize.pow(i as u32) % 2).collect_vec();

        if ar_list.iter().all(|(a, r)| match *r {
            'o' => a.iter().filter(|&i| key_list[*i] == 1).count() >= k,
            'x' => a.iter().filter(|&i| key_list[*i] == 1).count() < k,
            _ => true,
        }) {
            ans += 1;
        }
    }

    print!("{}", ans);
}
