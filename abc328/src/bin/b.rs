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
        let digits = m.to_string().chars().collect_vec();

        if !m.to_string().chars().all_equal() {
            continue;
        }

        let m1 = digits[0].to_digit(10).unwrap() as usize;
        let mut day = m1;
        let mut factor = 1;

        while d >= day {
            ans += 1;
            factor *= 10;
            day += m1 * factor;
        }
    }

    print!("{}", ans);
}
