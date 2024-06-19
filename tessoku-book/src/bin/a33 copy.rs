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
        a_list: [usize; n],
    }

    // 山が2つの時は物真似戦略が使える
    // 相手と同じ数を取ることで必勝
    // if n == 2 {
    //     if a_list[0] == a_list[1] {
    //         print!("Second");
    //     } else {
    //         print!("First");
    //     }

    //     return;
    // }

    // ニム和だと3つ以上の山でもいける
    if a_list.iter().fold(0, |acc, &a| acc ^ a) == 0 {
        print!("Second");
    } else {
        print!("First");
    }
}
