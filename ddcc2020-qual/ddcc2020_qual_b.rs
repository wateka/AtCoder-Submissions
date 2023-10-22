#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
use proconio::{input, marker::Chars};
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let sum = a.iter().sum::<i64>();
    let mut acc = 0;
    let mut diff = i64::MAX;
    
    for ai in a {
        acc += ai;
        let x = (2*acc - sum).abs();
        if x < diff {
            diff = x;
        } else {
            break;
        }
    }

    println!("{}", diff);
}
