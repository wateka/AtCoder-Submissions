#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::{collections::{HashMap, HashSet, BTreeMap, BTreeSet}, mem::swap};
use proconio::{input, marker::Chars};
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        mut p: [i64; n],
    }

    let mut i = n-1;
    while p[i-1] < p[i] {
        i -= 1;
    }

    let upper_i = p[i..n].upper_bound(&p[i-1]) + i - 1;
    let temp = p[upper_i];
    p[upper_i] = p[i-1];
    p[i-1] = temp;

    p[i..n].sort();
    p[i..n].reverse();

    for pi in p {
        print!("{} ", pi);
    }
    println!();
}
