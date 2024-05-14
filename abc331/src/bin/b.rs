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
        mut n: usize,
        s: usize,
        m: usize,
        l: usize,
    }

    let mut min = 10usize.pow(6);

    for s_cnt in 0..100 {
        for m_cnt in 0..100 {
            for l_cnt in 0..100 {
                if s_cnt * 6 + m_cnt * 8 + l_cnt * 12 >= n {
                    min = min.min(s_cnt * s + m_cnt * m + l_cnt * l);
                }
            }
        }
    }

    print!("{}", min);
}
