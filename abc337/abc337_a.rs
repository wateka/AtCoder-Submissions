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
        xys: [(i64, i64); n]
    }

    let mut x_sum = 0;
    let mut y_sum = 0;

    for (x, y) in xys {
        x_sum += x;
        y_sum += y;
    }

    if x_sum > y_sum {
        println!("Takahashi")
    } else if y_sum > x_sum {
        println!("Aoki")
    } else {
        println!("Draw")
    }
}
