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
        s: Chars
    }

    let s_length = s.len();

    print!(
        "{}",
        if s[0] == '<'
            && s[s_length - 1] == '>'
            && (s_length == 2 || s_length > 2 && s[1..(s_length - 1)].iter().all(|&c| c == '='))
        {
            "Yes"
        } else {
            "No"
        }
    );
}
