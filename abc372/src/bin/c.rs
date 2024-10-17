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
#[allow(unused_imports)]
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut s: String,
        xc_list: [(Usize1, String); q],
    }

    let mut abc_count = s.matches("ABC").count();

    let exists_affected_abc = |s: &str, x| {
        x >= 2 && s[x - 2..=x].eq("ABC")
            || x >= 1 && x <= n - 2 && s[x - 1..=x + 1].eq("ABC")
            || x <= n - 3 && s[x..=x + 2].eq("ABC")
    };

    for (x, c) in xc_list {
        if s[x..=x] != c {
            if exists_affected_abc(&s, x) {
                abc_count -= 1;
            }

            s.replace_range(x..=x, &c);

            if exists_affected_abc(&s, x) {
                abc_count += 1;
            }
        }

        println!("{}", abc_count);
    }
}
