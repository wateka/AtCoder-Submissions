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
        n: usize,
        cp: [(Usize1, i64); n],
        q: usize,
        lr: [(Usize1, Usize1); q],
    }

    let mut acc = vec![vec![0; 2]; n+1];

    for i in 0..n {
        acc[i+1][0] = acc[i][0];
        acc[i+1][1] = acc[i][1];

        let (c, p) = cp[i];
        acc[i+1][c] += p;
    }

    for (l, r) in lr {
        let aj = acc[r+1][0] - acc[l][0];
        let bj = acc[r+1][1] - acc[l][1];
        println!("{} {}", aj, bj);
    }
}
