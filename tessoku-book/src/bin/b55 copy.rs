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
use std::collections::BTreeSet;
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::iter::FromIterator;

#[fastout]
fn main() {
    input! {
        q: usize,
        queries: [(usize, usize); q],
    }

    let mut set = BTreeSet::new();

    for (t, x) in queries {
        match t {
            1 => {
                set.insert(x);
            }
            2 => {
                if set.is_empty() {
                    println!("-1");
                } else {
                    println!(
                        "{}",
                        min(
                            set.range(x..).next().unwrap_or(&usize::MAX).abs_diff(x),
                            set.range(..x)
                                .next_back()
                                .unwrap_or(&usize::MAX)
                                .abs_diff(x),
                        )
                    );
                }
            }
            _ => {}
        }
    }
}
