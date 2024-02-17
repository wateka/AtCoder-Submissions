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
        w: usize,
        wv: [(usize, i64); n]
    }

    let mut dp = vec![vec![0; w+1]; n+1];

    for i in 0..n {
        let (wi, vi) = wv[i];

        for j in 0..=w {
            dp[i+1][j] = dp[i+1][j].max(dp[i][j]);
            if (0..=w).contains(&(j+wi)) {
                dp[i+1][j+wi] = dp[i+1][j].max(dp[i][j]+vi);
            }
        }
    }

    let res = dp[n].iter().max().unwrap();
    putln!(res);
}
