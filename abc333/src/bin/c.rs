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
use std::usize;

fn main() {
    input! {
        mut n: usize,
    }

    let mut repunits = (0, 0, 0);

    let mut max = 0;

    loop {
        max += 1;

        repunits.2 = max;

        for rp1 in 1..=max {
            repunits.1 = rp1;

            for rp0 in 1..=rp1 {
                repunits.0 = rp0;

                n -= 1;

                if n == 0 {
                    print!(
                        "{}",
                        "1".repeat(repunits.0).parse::<usize>().unwrap()
                            + "1".repeat(repunits.1).parse::<usize>().unwrap()
                            + "1".repeat(repunits.2).parse::<usize>().unwrap()
                    );

                    return;
                }
            }
        }
    }
}
