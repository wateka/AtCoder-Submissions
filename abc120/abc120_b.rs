#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        a: usize,
        b: usize,
        k: usize,
    }

    let max = a.max(b);
    let mut common = BTreeSet::new();
    for i in 1..=max {
        if a % i == 0 && b % i == 0 {
            common.insert(i);
        }
    }

    let common = common.into_iter().collect_vec();
    eprintln!("{:?}", common);
    println!("{}", common[common.len() - k]);
}
