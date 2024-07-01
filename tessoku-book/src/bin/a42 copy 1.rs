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
        k: usize,
        ab_list: [(usize, usize); n],
    }

    // 2D累積和のための配列を初期化（サイズを101x101にして1-indexedにする）
    let mut cum_sum = vec![vec![0; 101]; 101];

    // 入力に基づいて2D配列にカウントを記録
    for &(a, b) in &ab_list {
        cum_sum[a][b] += 1;
    }

    // 2D累積和を計算
    for i in 1..=100 {
        for j in 1..=100 {
            cum_sum[i][j] += cum_sum[i - 1][j] + cum_sum[i][j - 1] - cum_sum[i - 1][j - 1];
        }
    }

    // 最大値を求める
    let mut max_count = 0;
    for a in 1..=100 {
        for b in 1..=100 {
            let right = (a + k).min(100);
            let top = (b + k).min(100);
            let count = cum_sum[right][top] - cum_sum[a - 1][top] - cum_sum[right][b - 1]
                + cum_sum[a - 1][b - 1];
            max_count = max_count.max(count);
        }
    }

    println!("{}", max_count);
}
