#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }

    // let a_tree: BTreeSet<i64> = a.clone().into_iter().collect();

    a.sort();
    a.reverse();

    let mut k_count = vec![0; n];
    let mut i = 0;
    k_count[0] = 1;

    for (ai_prev, ai) in a.into_iter().tuple_windows() {
        if ai_prev != ai {
            i += 1;
        }
        k_count[i] += 1;
    }

    for k in k_count {
        println!("{}", k);
    }
}
