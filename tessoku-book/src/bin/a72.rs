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
        h: usize,
        w: usize,
        k: usize,
        grid: [Chars; h],
    }

    let mut min = 1000;

    for ki in 0..=k {
        for combination in (0..h).combinations(ki) {
            let mut select_col = vec![];

            if ki < k {
                let mut col_white_cnt = vec![0; w];

                for (i, row) in grid.iter().enumerate() {
                    if combination.contains(&i) {
                        continue;
                    }

                    for (j, &c) in row.iter().enumerate() {
                        if c == '.' {
                            col_white_cnt[j] += 1;
                        }
                    }
                }

                let mut cnt_i = col_white_cnt.iter().enumerate().collect_vec();

                cnt_i.sort_unstable_by_key(|&(_, &cnt)| cnt);

                for _ in 0..k - ki {
                    select_col.push(*cnt_i.pop().unwrap().1);
                }
            }

            let mut count = 0;

            for (i, row) in grid.iter().enumerate() {
                if combination.contains(&i) {
                    continue;
                }

                for (j, &c) in row.iter().enumerate() {
                    if select_col.contains(&j) {
                        continue;
                    }

                    if c == '.' {
                        count += 1;
                    }
                }
            }

            min = min.min(count);
        }
    }

    print!("{}", h * w - min);
}
