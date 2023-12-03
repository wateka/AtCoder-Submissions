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
        h: usize,
        w: usize,
        a: [[i64; w]; h],
    }

    let mut hsum = vec![0; h];
    let mut wsum = vec![0; w];
    for i in 0..h {
        for j in 0..w {
            hsum[i] += a[i][j];
            wsum[j] += a[i][j];
        }
    }

    for i in 0..h {
        for j in 0..w {
            print!("{} ", hsum[i] + wsum[j] - a[i][j]);
        }
        println!()
    }
}
