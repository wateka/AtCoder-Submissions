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
        q: usize,
        queries: [(usize, usize, usize); q],
    }

    let mut dsu = Dsu::new(n);
    for (t, u, v) in queries {
        if t == 0 {
            dsu.merge(u, v);
        } else {
            let res = if dsu.same(u, v) { 1 } else { 0 };
            putln!(res);
        }
    }
}
