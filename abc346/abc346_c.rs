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
        k: i64,
        a: [i64; n],
    }

    let mut res = k * (k+1) / 2;
    let mut set = HashSet::new();
    for ai in a {
        if ai <= k && !set.contains(&ai) {
            res -= ai;
            set.insert(ai);
        }
    }
    putln!(res);
}
