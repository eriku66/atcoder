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

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    for (i, (&cs, &ct)) in s.iter().zip(&t).enumerate() {
        if cs != ct {
            print!("{}", i + 1);
            return;
        }
    }

    if s.len() != t.len() {
        print!("{}", s.len().min(t.len()) + 1);
        return;
    }

    print!("0");
}
