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

    let mut graph = vec![Vec::with_capacity(n); n];

    for (a, b, c) in abc_list {
        graph[a].push((b, c));
        graph[b].push((a, c));
    }

    let mut dists = vec![-1; n];
    let mut heap = BinaryHeap::new();

    heap.push((Reverse(0), 0));

    while let Some((Reverse(min_d), min_p)) = heap.pop() {
        if dists[min_p] != -1 {
            continue;
        }

        dists[min_p] = min_d;

        for &(p, d) in graph[min_p].iter().filter(|&&(p, _)| dists[p] == -1) {
            heap.push((Reverse(min_d + d), p))
        }
    }

    let mut ans = vec![n - 1];

    while let Some(&(p, _)) = graph[*ans.last().unwrap()]
        .iter()
        .find(|&&(p, d)| dists[*ans.last().unwrap()] - d == dists[p])
    {
        ans.push(p);
    }

    print!("{}", ans.iter().rev().map(|&p| p + 1).join(" "));
}
