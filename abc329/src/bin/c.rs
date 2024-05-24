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
        s: Chars,
    }

    let mut map: HashMap<char, usize> = HashMap::with_capacity(26);

    let mut cnt = 1;

    for i in 0..n {
        if i == n - 1 || s[i] != s[i + 1] {
            map.entry(s[i])
                .and_modify(|v| *v = (*v).max(cnt))
                .or_insert(cnt);
            cnt = 1;
        } else {
            cnt += 1;
        }
    }

    print!("{}", map.values().sum::<usize>());
}
