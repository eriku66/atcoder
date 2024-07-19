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

// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab_list: [(Usize1, Usize1); m],
    }

    let mut g = vec![Vec::with_capacity(n); n + 1];

    for (a, b) in ab_list {
        g[a].push(b);
        g[b].push(a);
    }

    g.iter_mut().for_each(|e| e.sort_unstable());

    let mut dist = vec![-1; n];
    let mut queue = VecDeque::with_capacity(n);

    dist[0] = 0;
    queue.push_front(0);

    while !queue.is_empty() {
        let pos = queue.pop_front().unwrap();

        for &e in &g[pos] {
            if dist[e] == -1 {
                queue.push_back(e);
                dist[e] = dist[pos] + 1;
            }
        }
    }

    for d in dist {
        println!("{}", d);
    }
}
