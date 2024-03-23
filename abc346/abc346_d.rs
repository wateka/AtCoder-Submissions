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
        s: Chars,
        c: [i64; n],
    }

    let mut cost01 = vec![0; n];
    let mut cost10 = vec![0; n];
    for i in 0..n {
        if (i%2==0 && s[i]=='0') || (i%2==1 && s[i]=='1') {
            cost01[i] = c[i];
        } else {
            cost10[i] = c[i];
        }
    }

    let mut acc01_l = vec![0; n+1];
    let mut acc10_l = vec![0; n+1];
    let mut acc01_r = vec![0; n+1];
    let mut acc10_r = vec![0; n+1];
    for i in 0..n {
        acc01_l[i+1] = acc01_l[i] + cost01[i];
        acc10_l[i+1] = acc10_l[i] + cost10[i];
        acc01_r[n-1-i] = acc01_r[n-i] + cost01[n-1-i];
        acc10_r[n-1-i] = acc10_r[n-i] + cost10[n-1-i];
    }

    let mut sum0 = vec![0; n+1];
    let mut sum1 = vec![0; n+1];
    for i in 0..=n {
        sum0[i] = acc01_l[i] + acc10_r[i];
        sum1[i] = acc10_l[i] + acc01_r[i];
    }

    sum0[0] = i64::MAX;
    sum0[n] = i64::MAX;
    sum1[0] = i64::MAX;
    sum1[n] = i64::MAX;

    let res0 = sum0.iter().min().unwrap();
    let res1 = sum1.iter().min().unwrap();
    putln!(res0.min(res1));

    eprintln!("{:?}", acc01_l);
    eprintln!("{:?}", acc10_r);
}
