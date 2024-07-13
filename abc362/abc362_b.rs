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
        a1: i64,
        a2: i64,
        b1: i64,
        b2: i64,
        c1: i64,
        c2: i64,
    }

    let ab = (a1 - b1) * (a1 - b1) + (a2 - b2) * (a2 - b2);
    let bc = (c1 - b1) * (c1 - b1) + (c2 - b2) * (c2 - b2);
    let ca = (a1 - c1) * (a1 - c1) + (a2 - c2) * (a2 - c2);

    let ans = ab + bc == ca || bc + ca == ab || ca + ab == bc;

    yes_no!(ans);
}
