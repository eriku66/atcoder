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
        t: isize,
        s: Chars,
        x_list: [isize; n],
    }

    let mut p_list = Vec::new();
    let mut m_list = Vec::new();

    for (i, &c) in s.iter().enumerate() {
        match c {
            '0' => m_list.push(x_list[i]),
            '1' => p_list.push(x_list[i]),
            _ => {}
        }
    }

    if p_list.is_empty() || m_list.is_empty() {
        print!("0");
        return;
    }

    p_list.sort_unstable();
    m_list.sort_unstable();

    let mut ans = 0;

    for p in p_list {
        let l = m_list.binary_search(&p).map_or_else(|e| e, |v| v + 1);
        let r = m_list
            .binary_search(&(t * 2 + p))
            .map_or_else(|e| e, |v| v + 1);
        ans += r - l;
    }

    print!("{}", ans);
}
