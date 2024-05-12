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
        mut a: usize,
        mut b: usize,
        mut c: usize,
        mut d: usize,
        mut e: usize,
        mut f: usize,
        n: usize,
        mut x_list: [usize; n],
    }

    for x in x_list.iter_mut() {
        let f_use = f.min(*x / 500);
        f -= f_use;
        *x -= f_use * 500;

        let e_use = e.min(*x / 100);
        e -= e_use;
        *x -= e_use * 100;

        let d_use = d.min(*x / 50);
        d -= d_use;
        *x -= d_use * 50;

        let c_use = c.min(*x / 10);
        c -= c_use;
        *x -= c_use * 10;

        let b_use = b.min(*x / 5);
        b -= b_use;
        *x -= b_use * 5;

        let a_use = a.min(*x);
        a -= a_use;
        *x -= a_use;

        if *x != 0 {
            print!("No");
            return;
        }
    }

    print!("Yes");
}
