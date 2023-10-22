#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        xys: [(f64, f64); n],
    }

    let c = (1..=n).fold(1, |acc, i| acc * i);

    let mut ave = 0.0;
    for p in (0..n).permutations(n) {
        let mut d_sum = 0.0;
        for w in p.windows(2) {
            let (i, j) = (w[0], w[1]);
            let (xi, yi) = xys[i];
            let (xj, yj) = xys[j];
            let d = ((xj - xi).powi(2) + (yj - yi).powi(2)).sqrt();

            d_sum += d;
        }

        ave += d_sum / c as f64;
    }

    println!("{}", ave);
}
