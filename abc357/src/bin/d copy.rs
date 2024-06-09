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

    // 公比
    let r = ModInt998244353::new(10usize.pow(n.ilog10() + 1));

    /**
     * 等比数列の和の公式 S = a(r^n - 1) / r - 1
     *
     * a: 初項
     * r: 公比
     * n: 項数
     */
    print!("{}", n * (r.pow(n) - 1) / (r - 1));
}
