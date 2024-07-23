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
use std::{cmp::Reverse, collections::BinaryHeap};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        abc_list: [(Usize1, Usize1, isize); m],
    }

    let mut g = vec![Vec::with_capacity(n); n];

    for (a, b, c) in abc_list {
        g[a].push((b, c));
        g[b].push((a, c));
    }

    let mut cur = vec![-1; n];
    let mut queue = BinaryHeap::new();

    queue.push((Reverse(0), 0));

    while let Some((Reverse(min_w), min_p)) = queue.pop() {
        if cur[min_p] != -1 {
            continue;
        }

        cur[min_p] = min_w;

        for &(p, w) in g[min_p].iter() {
            if cur[p] != -1 {
                continue;
            }

            queue.push((Reverse(w + min_w), p));
        }
    }

    for dist in cur {
        println!("{}", dist);
    }
}
