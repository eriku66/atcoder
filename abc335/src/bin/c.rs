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
        q: usize,
    }

    let mut head_points = (1..=n).rev().map(|i| (i as i32, 0)).collect::<Vec<_>>();

    for _ in 0..q {
        input! {t: char};

        if t == '1' {
            input! {c: char}

            let mut new_head_point = *head_points.last().unwrap();

            match c {
                'R' => new_head_point.0 += 1,
                'L' => new_head_point.0 -= 1,
                'U' => new_head_point.1 += 1,
                'D' => new_head_point.1 -= 1,
                _ => {}
            }

            head_points.push(new_head_point);
        } else if t == '2' {
            input! {i: usize};

            let point = head_points[head_points.len() - i];

            println!("{} {}", point.0, point.1);
        }
    }
}
