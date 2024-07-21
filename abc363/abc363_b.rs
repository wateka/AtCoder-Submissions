#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]

use ac_library::{dsu::Dsu, modint::Mod1000000007 as Mint10, modint::ModInt998244353 as Mint9};
use itertools::{iproduct, izip, Itertools};
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use superslice::Ext;
macro_rules! yes_no {
    ($condition: expr) => {
        println!("{}", if $condition { "Yes" } else { "No" })
    };
}
macro_rules! putln {
    ($value: expr) => {
        println!("{}", $value)
    };
}

#[fastout]
fn main() {
    input! {
        n: usize,
        t: i64,
        p: usize,
        mut l: [i64; n],
    }

    l.sort();
    l.reverse();

    let ans = (t - l[p - 1]).max(0);

    putln!(ans);
}
