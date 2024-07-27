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
        s_list: [String; n],
    }

    print!(
        "{}",
        if s_list
            .windows(2)
            .take(n - 2)
            .any(|w| { w.iter().all(|s| s.eq("sweet")) })
        {
            "No"
        } else {
            "Yes"
        }
    );
}
