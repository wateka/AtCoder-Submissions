#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
use proconio::{input, marker::{Chars, Usize1}};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }

    let mut gen = vec![0; 2*n+1];

    for (i, ai) in a.into_iter().enumerate() {
        gen[2*i+1] = gen[ai]+1;
        gen[2*i+2] = gen[ai]+1;
    }

    for g in gen {
        println!("{}", g);
    }
}
