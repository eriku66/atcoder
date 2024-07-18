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
        mut ab_list: [(usize, usize); m],
    }

    ab_list.sort_unstable();

    let mut graph = vec![vec![]; n + 1];

    for (a, b) in ab_list {
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut visited = Vec::with_capacity(n);

    dfs(
        1,
        n,
        &graph,
        &mut visited,
        &mut vec![false; n + 1],
        &mut false,
    );

    print!("{}", visited.iter().join(" "));
}

fn dfs(
    pos: usize,
    n: usize,
    graph: &[Vec<usize>],
    visited: &mut Vec<usize>,
    is_visited: &mut [bool],
    reach: &mut bool,
) {
    visited.push(pos);
    is_visited[pos] = true;

    if pos == n {
        *reach = true;
        return;
    }

    for &next in graph[pos].iter() {
        if !is_visited[next] {
            dfs(next, n, graph, visited, is_visited, reach);

            if *reach {
                break;
            }

            visited.pop();
        }
    }
}
