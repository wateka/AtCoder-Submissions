#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]

use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
use ac_library::{dsu::Dsu, modint::ModInt998244353 as Mint9, modint::Mod1000000007 as Mint10};
use itertools::{Itertools, izip, iproduct};
use superslice::Ext;
use proconio::{fastout, input, marker::{Chars, Usize1}};
macro_rules! yes_no { ($condition: expr) => {println!("{}", if $condition {"Yes"} else {"No"})} }
macro_rules! putln { ($value: expr) => {println!("{}", $value)} }

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        m: usize,
        queries: [(usize, Usize1, usize); m],
    }

    let mut colors = vec![0; m+1];
    let mut row = vec![0; h];
    let mut col = vec![0; w];

    for i in 0..m {
        let (ti, ai, xi) = queries[i];
        colors[i+1] = xi;
        if ti == 1 {
            row[ai] = i+1;
        } else {
            col[ai] = i+1;
        }
    }

    row.sort();
    col.sort();

    let mut counts = vec![0; 200001];
    for &ri in row.iter() {
        if ri == 0 {
            continue;
        }
        counts[colors[ri]] += col.upper_bound(&ri);
    }
    for &ci in col.iter() {
        if ci == 0 {
            continue;
        }
        counts[colors[ci]] += row.upper_bound(&ci);
    }

    counts[0] = 0;
    let counts_sum: usize = counts.iter().sum();
    counts[0] = h * w - counts_sum;
    // eprintln!("{} {}", h*w, counts_sum);

    let ans_n = counts.iter().filter(|&&count| count > 0).count();
    putln!(ans_n);
    for (i, count) in counts.into_iter().enumerate() {
        if count > 0 {
            println!("{} {}", i, count);
        }
    }
}
