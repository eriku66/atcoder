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
        mut n: usize,
    }

    if n == 1 {
        print!("0");
        return;
    }

    n -= 2;

    let mut count = 0;
    let mut nine = 9;

    while n >= nine {
        n -= nine;
        count += 1;

        if n >= nine {
            n -= nine;
            count += 1;
        }

        nine *= 10;
    }

    n += 10_usize.pow(count / 2);

    let mut str = n.to_string();
    let rev = str.chars().rev().collect::<String>();

    if count % 2 == 0 {
        str.pop();
    }

    println!("{}{}", str, rev);
}
