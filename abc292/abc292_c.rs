#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
    }

    let mut pc = vec![0usize; n];

    for i in 1..n {
        let sqrt_i = (i as f64).sqrt().ceil() as usize;
        let mut set = HashSet::new();

        for j in 1..=sqrt_i {
            if i % j == 0 {
                set.insert(j);
                set.insert(i / j);
            }
        }

        pc[i] = set.len();
    }

    // eprintln!("{:?}", pc);

    let h = (n+1)/2;
    let sum1 = (1..h).map(|i| pc[i]*pc[n-i]).sum::<usize>() * 2;
    let sum2 = if n % 2 == 0 { pc[h] * pc[h] } else { 0 };

    println!("{}", sum1 + sum2);
}
