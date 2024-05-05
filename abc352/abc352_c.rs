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
        ab: [(i64, i64); n],
    }

    let asum = ab.iter().map(|(a, b)| a).sum::<i64>();
    let mut ans = 0;

    for i in 0..n {
        let (ai, bi) = ab[i];
        let x = asum - ai + bi;
        ans = ans.max(x);
    }

    putln!(ans);
}
