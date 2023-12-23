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

fn calc_k(a: i64, m: i64, p: i64) -> i64 {
    (p - a + m) / m
}

#[fastout]
fn main() {
    input! {
        a: i64,
        m: i64,
        l: i64,
        r: i64,
    }

    let rl = (l-a)%m;
    let cl = (l-a)/m - if l-a<0 && (rl)!=0 { 1 } else { 0 };
    let rr = (r-a)%m;
    let cr = (r-a)/m - if r-a<0 && (rr)!=0 { 1 } else { 0 };

    if rr == 0 && rl == 0 {
        putln!(cr - cl + 1)
    } else if rr == 0 {
        putln!(cr - cl)
    } else if rl == 0 {
        putln!(cr - cl + 1)
    } else {
        putln!(cr - cl)
    }
}
