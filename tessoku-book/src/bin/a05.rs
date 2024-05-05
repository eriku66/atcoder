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
        n: isize,
        k: isize,
    }

    // My solution

    // let mut result = 0;

    // for a in 1..=n {
    //     if k - a < a {
    //         break;
    //     }

    //     for b in a..=n {
    //         let c = k - a - b;

    //         if c > n {
    //             continue;
    //         }

    //         if c < b {
    //             break;
    //         }

    //         result += if a * 3 == k {
    //             1
    //         } else if a == b || b == c {
    //             3
    //         } else {
    //             6
    //         };
    //     }
    // }

    // print!("{}", result);

    // Simple

    let mut result = 0;

    for x in 1..=n {
        for y in 1..=n {
            let z = k - x - y;

            if z >= 1 && z <= n {
                result += 1;
            }
        }
    }

    print!("{}", result);
}
