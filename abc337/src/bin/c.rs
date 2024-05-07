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
        a_list: [isize; n],
    }

    let mut map = HashMap::with_capacity(n);

    for (i, &a) in a_list.iter().enumerate() {
        if a == -1 {
            map.entry(0).or_insert(i + 1);
        } else if a >= 1 {
            map.entry(a as usize).or_insert(i + 1);
        }
    }

    let mut result = Vec::with_capacity(n);

    let mut next = 0;

    for _ in 0..n {
        next = *map.get(&next).unwrap();

        result.push(next);
    }

    print!("{}", result.iter().join(" "));
}
