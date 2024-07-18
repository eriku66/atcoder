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
        m: usize,
        mut ab_list: [(Usize1, Usize1); m],
    }

    ab_list.sort_unstable();

    let mut g = vec![Vec::with_capacity(n); n];

    for (a, b) in ab_list {
        g[a].push(b);
        g[b].push(a);
    }

    let mut visited = vec![false; n];

    dfs(0, &g, &mut visited);

    print!(
        "{}",
        if visited.iter().all(|&b| b) {
            "The graph is connected."
        } else {
            "The graph is not connected."
        }
    );
}

fn dfs(pos: usize, g: &[Vec<usize>], visited: &mut [bool]) {
    visited[pos] = true;

    for &next in &g[pos] {
        if !visited[next] {
            dfs(next, g, visited);
        }
    }
}
