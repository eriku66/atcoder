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
        d: usize,
        mut xy_list: [(Usize1, usize); n],
    }

    xy_list.sort_unstable_by_key(|(_, y)| *y);

    let mut ans = 0;

    for i in 0..d {
        // 仕事を選択できない場合もあることに気を付ける
        if let Some(max_i) = xy_list.iter().rposition(|&(x, _)| x <= i) {
            ans += xy_list.remove(max_i).1;
        }
    }

    print!("{}", ans);
}
