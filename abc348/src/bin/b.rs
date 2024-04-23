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
        points: [(isize, isize); n],
    }

    for (x1, y1) in &points {
        let mut farthest_point_index = 0;
        let mut squared_distance_to_farthest_point = 0;

        for (index, (x2, y2)) in points.iter().enumerate() {
            let squared_distance = x1.abs_diff(*x2).pow(2) + y1.abs_diff(*y2).pow(2);

            if squared_distance > squared_distance_to_farthest_point {
                farthest_point_index = index;
                squared_distance_to_farthest_point = squared_distance;
            }
        }

        println!("{}", farthest_point_index + 1);
    }
}
