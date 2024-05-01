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
        s: Chars,
    }

    let mut mc = '.';
    let mut map: HashMap<char, usize> = HashMap::with_capacity(3);

    for &c in s.iter().take(3) {
        let cnt = map.entry(c).or_insert(0);
        *cnt += 1;
        if *cnt > 1 {
            mc = c;
            break;
        }
    }

    print!("{}", s.iter().position(|&c| c != mc).unwrap() + 1);
}
