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
#[allow(unused_imports)]
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        m: usize,
        a_list: [Usize1; m],
    }

    let set: HashSet<usize> = HashSet::from_iter(a_list.into_iter());

    let mut ans = Vec::new();
    let mut diff = 0;

    for i in (0..n).rev() {
        if set.contains(&i) {
            diff = 0;
        } else {
            diff += 1;
        }

        ans.push(diff);
    }

    for d in ans.into_iter().rev() {
        println!("{}", d);
    }
}
