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
        t: usize,
        ab_list: [(Usize1, usize); t],
    }

    let mut m = HashMap::new();
    let mut count = 1;
    m.insert(0, n);

    let mut l = vec![0; n];

    for (a, b) in ab_list {
        let v = m.get_mut(&l[a]).unwrap();
        *v -= 1;

        if *v == 0 {
            count -= 1;
            m.remove(&l[a]);
        }

        l[a] += b;

        *m.entry(l[a]).or_insert_with(|| {
            count += 1;
            0
        }) += 1;

        println!("{}", count);
    }
}
