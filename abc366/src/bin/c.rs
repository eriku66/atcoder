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
#[allow(unused_imports)]
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        q: usize,
    }

    let mut count = 0;

    let mut balls = HashMap::new();

    for _ in 0..q {
        input! {t: usize}

        match t {
            1 => {
                input! {x: usize}
                *balls.entry(x).or_insert_with(|| {
                    count += 1;
                    0
                }) += 1;
            }
            2 => {
                input! {x:usize}
                let c = balls.get_mut(&x).unwrap();

                if *c == 1 {
                    balls.remove(&x);
                    count -= 1;
                } else {
                    *c -= 1;
                }
            }
            3 => println!("{}", count),
            _ => {}
        }
    }
}
