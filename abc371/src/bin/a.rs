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
        s_ab: char,
        s_ac: char,
        s_bc: char,
    }

    if s_ab == s_ac && s_ab == s_bc {
        print!("B");
    } else if s_ab == s_ac && s_ac != s_bc {
        print!("C");
    } else {
        print!("A");
    }
}
