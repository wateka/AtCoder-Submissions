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
        mut r: [i64; n],
        queries: [i64; q],
    }

    r.sort();

    let mut acc_r = vec![0; n];
    acc_r[0] = r[0];
    for i in 1..n {
        acc_r[i] = acc_r[i-1] + r[i];
    }

    for query in queries {
        let i = acc_r.upper_bound(&query);
        putln!(i);
    }
}
