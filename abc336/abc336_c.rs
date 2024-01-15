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

const D: [char; 5] = ['0', '2', '4', '6', '8'];

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut v = vec![];

    let mut n = n-1;

    if n == 0 {
        putln!("0");
    }

    while n > 0 {
        v.push(D[n % 5]);
        n /= 5;
    }

    putln!(v.into_iter().rev().join(""));
}
