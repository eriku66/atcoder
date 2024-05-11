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
        _n: usize,
        m: usize,
        s: Chars,
    }

    let mut plain_t = m;

    let mut logo_t = 0;

    let mut max_logo_t = 0;

    for c in s {
        match c {
            '0' => {
                plain_t = m;

                if max_logo_t < -logo_t {
                    max_logo_t = -logo_t;
                }

                logo_t = 0;
            }
            '1' => {
                if plain_t >= 1 {
                    plain_t -= 1;
                } else {
                    logo_t -= 1;
                }
            }
            '2' => {
                logo_t -= 1;
            }
            _ => {}
        }
    }

    if max_logo_t < -logo_t {
        max_logo_t = -logo_t;
    }

    print!("{}", max_logo_t);
}
