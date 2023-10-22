#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        t: i64,
        a: [i64; n],
    }

    let a_sum = a.iter().sum::<i64>();
    let a_acc = a.iter().scan(0, |acc, ai| {
        *acc += ai;
        Some(*acc-ai)
    }).collect_vec();

    let t = t % a_sum;

    let mut i = 0;
    for ai_acc in &a_acc {
        if t < *ai_acc {
            break;
        }
        i += 1;
    }

    println!("{} {}", i, t-a_acc[i-1]);
}
