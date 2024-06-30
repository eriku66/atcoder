#[allow(unused_imports)]
use itertools::{iproduct, Itertools};
#[allow(unused_imports)]
use num_traits::pow;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Ordering;
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::iter::FromIterator;

#[fastout]
fn main() {
    input! {
        mut x: usize,
        mut y: usize,
    }

    let mut ans = Vec::new();

    loop {
        if x == 1 && y == 1 {
            break;
        }

        ans.push((x, y));

        match x.cmp(&y) {
            Ordering::Greater => x -= y,
            Ordering::Less => y -= x,
            Ordering::Equal => {}
        }
    }

    println!("{}", ans.len());

    for (x, y) in ans.iter().rev() {
        println!("{} {}", x, y);
    }
}
