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
        k: usize,
        mut a_list: [usize; n],
    }

    let mut first_half = HashSet::new();
    first_half.insert(0);
    let mut second_half = HashSet::new();
    second_half.insert(0);

    for &a in a_list.iter().take(n / 2) {
        let mut set = HashSet::new();

        for &b in &first_half {
            set.insert(b + a);
        }

        set.insert(a);

        first_half.extend(set);
    }

    for &a in a_list.iter().skip(n / 2) {
        let mut set = HashSet::new();

        for &b in &second_half {
            set.insert(b + a);
        }

        set.insert(a);

        second_half.extend(set);
    }

    for t in first_half {
        if second_half.contains(&(k - t)) {
            print!("Yes");
            return;
        }
    }

    print!("No");
}
