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
        h: [i64; n],
    }

    let mut dp = vec![i64::MAX; n];
    dp[0] = 0;
    for i in 0..n {
        if i + 1 < n {
            dp[i+1] = dp[i+1].min(dp[i] + (h[i+1]-h[i]).abs())
        }
        if i + 2 < n {
            dp[i+2] = dp[i+2].min(dp[i] + (h[i+2]-h[i]).abs())
        }
    }

    putln!(dp[n-1])
}
