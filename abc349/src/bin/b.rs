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
        s: String,
    }

    let mut char_counts: HashMap<char, usize> = HashMap::new();

    for c in s.chars() {
        let count = char_counts.entry(c).or_insert(0);
        *count += 1;
    }

    let mut char_count_counts: HashMap<usize, usize> = HashMap::new();

    for (_, char_count) in char_counts {
        let count = char_count_counts.entry(char_count).or_insert(0);
        *count += 1;

        if *count > 2 {
            print!("No");
            return;
        }
    }

    for (_, count) in char_count_counts {
        if count == 1 {
            print!("No");
            return;
        }
    }

    print!("Yes");
}
