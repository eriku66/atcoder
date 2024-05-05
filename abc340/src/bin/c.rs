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
    }

    let mut memo = HashMap::new();
    memo.insert(1, 0);

    fn ope(x: usize, memo: &mut HashMap<usize, usize>) -> usize {
        if let Some(v) = memo.get(&x) {
            *v
        } else {
            let v = x + ope(x.div_euclid(2), memo) + ope((x + 1).div_euclid(2), memo);
            memo.insert(x, v);
            v
        }
    }

    print!("{}", ope(n, &mut memo));
}
