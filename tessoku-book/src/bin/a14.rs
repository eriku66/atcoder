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
use std::collections::BTreeSet;
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::iter::FromIterator;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        mut a_list: [usize; n],
        mut b_list: [usize; n],
        mut c_list: [usize; n],
        mut d_list: [usize; n],
    }

    let mut ab_list = BTreeSet::new();

    for a in a_list {
        for &b in &b_list {
            ab_list.insert(a + b);
        }
    }

    let mut cd_list = BTreeSet::new();

    for c in c_list {
        for &d in &d_list {
            cd_list.insert(c + d);
        }
    }

    for ab in ab_list {
        if cd_list.contains(&(k - ab)) {
            print!("Yes");
            return;
        }
    }

    print!("No");
}
