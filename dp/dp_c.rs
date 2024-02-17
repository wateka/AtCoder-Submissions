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
        abcs: [[i64; 3]; n],
    }

    let mut dp = vec![vec![0; 3]; n+1];

    for i in 1..=n {
        for (j, k) in iproduct!(0..3, 0..3) {
            if j == k {
                continue;
            }
            dp[i][j] = dp[i][j].max(dp[i-1][k] + abcs[i-1][j]);
        }
    }

    let ans = dp[n].iter().max().unwrap();
    putln!(ans)
}
