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
        mut a: [i64; n],
        st: [(i64, i64); n-1],
    }

    for i in 0..(n-1) {
        let (si, ti) = st[i];
        let c = a[i] / si;
        a[i+1] += ti * c;
    }

    putln!(a[n-1])
}
