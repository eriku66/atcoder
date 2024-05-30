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
    const N: usize = 9;

    input! {
        a: [[usize; N]; N],
    }

    for row in &a {
        if row.iter().unique().count() != N {
            print!("No");
            return;
        }
    }

    for i in 0..N {
        if a.iter().map(|row| row[i]).unique().count() != N {
            print!("No");
            return;
        }
    }

    for i in (0..N).step_by(3) {
        for j in (0..N).step_by(3) {
            if a.iter()
                .skip(i)
                .take(3)
                .flat_map(|row| row.iter().skip(j).take(3))
                .unique()
                .count()
                != N
            {
                print!("No");
                return;
            };
        }
    }

    print!("Yes");
}
