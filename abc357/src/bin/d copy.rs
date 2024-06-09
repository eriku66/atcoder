use ac_library::ModInt998244353;
#[allow(unused_imports)]
use itertools::{iproduct, Itertools};
use num::Integer;
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
        n: u64,
    }

    let r = ModInt998244353::new(10usize.pow(n.ilog10() + 1));
    print!("{}", (r.pow(n) - 1) * n / (r - 1));
}
