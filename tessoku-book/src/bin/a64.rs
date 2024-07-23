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
use std::{cmp::Reverse, collections::BinaryHeap};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        abc_list: [(Usize1, Usize1, isize); m],
    }
}
