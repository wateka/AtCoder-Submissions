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
        a: [[usize; n]; n]
    }

    let mut edges = vec![vec![]; n];
    for i in 0..n {
        for j in 0..n {
            let aij = a[i][j];
            if aij == 1 {
                edges[i].push(j);
            }
        }
    }

    for i in 0..n {
        let res = edges[i].iter().map(|j| j+1).join(" ");
        putln!(res);
    }
}
