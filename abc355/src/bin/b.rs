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
        a_list: [usize; n],
        b_list: [usize; m],
    }

    if n == 1 {
        print!("No");
        return;
    }

    let al = a_list.iter().map(|a| (a, 'a')).collect_vec();
    let bl = b_list.iter().map(|a| (a, 'b')).collect_vec();

    let mut l = [al, bl].concat();
    l.sort_unstable();

    for w in l.windows(2) {
        if w[0].1 == 'a' && w[1].1 == 'a' {
            print!("Yes");
            return;
        }
    }

    print!("No");
}
