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
        d_list: [usize; n],
    }

    let mut ans = 0;

    for (i, &d) in d_list.iter().enumerate() {
        let m = i + 1;

        if !m.to_string().chars().all_equal() {
            continue;
        }

        let m1 = m.to_string().chars().next().unwrap().to_digit(10).unwrap() as usize;
        let mut day = m1;
        let mut cnt = 1;

        while d >= day {
            ans += 1;
            day += m1 * 10usize.pow(cnt);
            cnt += 1;
        }
    }

    print!("{}", ans);
}
