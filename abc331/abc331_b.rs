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
        n: i64,
        s: i64,
        m: i64,
        l: i64,
    }

    let mut ans = i64::MAX;
    for (i, j, k) in iproduct!(0..=20, 0..=20, 0..=20) {
        if 6*i + 8*j + 12*k >= n {
            ans = ans.min(s*i + m*j + l*k);
        }
    }
    putln!(ans)
}
