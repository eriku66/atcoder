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
        a_list: [i128; n],
        uvb_list: [(Usize1, Usize1, i128); m],
    }

    let mut graph = vec![Vec::with_capacity(n); n];

    for (u, v, b) in uvb_list {
        graph[u].push((v, b));
        graph[v].push((u, b));
    }

    let mut weights = vec![-1; n];
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(a_list[0]), 0));

    while let Some((Reverse(min_w), min_p)) = heap.pop() {
        if weights[min_p] != -1 {
            continue;
        }

        weights[min_p] = min_w;

        for &(p, w) in graph[min_p].iter().filter(|&&(p, _)| weights[p] == -1) {
            heap.push((Reverse(weights[min_p] + w + a_list[p]), p));
        }
    }

    print!("{}", weights.iter().skip(1).join(" "));
}
