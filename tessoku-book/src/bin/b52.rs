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
        x: Usize1,
        mut a: Chars,
    }

    let mut queue = VecDeque::new();
    queue.push_front(x);
    a[x] = '@';

    while !queue.is_empty() {
        let pos = queue.pop_front().unwrap();

        if pos > 0 && a[pos - 1] == '.' {
            a[pos - 1] = '@';
            queue.push_back(pos - 1);
        }

        if pos + 1 < n && a[pos + 1] == '.' {
            a[pos + 1] = '@';
            queue.push_back(pos + 1);
        }
    }

    print!("{}", a.iter().collect::<String>());
}
