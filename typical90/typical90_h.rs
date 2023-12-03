#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]

use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
use ac_library::{dsu::Dsu, modint::ModInt998244353 as Mint9, modint::ModInt1000000007 as Mint10};
use itertools::{Itertools, izip, iproduct};
use superslice::Ext;
use proconio::{fastout, input, marker::{Chars, Usize1}};
macro_rules! yes_no { ($condition: expr) => {println!("{}", if $condition {"Yes"} else {"No"})} }
macro_rules! putln { ($value: expr) => {println!("{}", $value)} }

const S: [char; 7] = ['a', 't', 'c', 'o', 'd', 'e', 'r'];

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut dp = vec![[Mint10::new(0); 8]; n+1];
    dp[0][0] = Mint10::new(1);
    for i in 0..n {
        for j in 0..8 {
            dp[i+1][j] = dp[i][j] + dp[i+1][j];
            if j < 7 && s[i] == S[j] {
                dp[i+1][j+1] = dp[i][j] + dp[i+1][j+1];
            }
        }
    }
    putln!(dp[n][7])
}
