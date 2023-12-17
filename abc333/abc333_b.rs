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

fn to(c: char) -> i64 {
    match c {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        'E' => 4,
        _ => unreachable!(),
    }
}

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let s1 = to(s[0]);
    let s2 = to(s[1]);
    let t1 = to(t[0]);
    let t2 = to(t[1]);

    let ds = (5 + (s1 - s2).abs()) % 5;
    let dt = (5 + (t1 - t2).abs()) % 5;

    yes_no!(ds == dt || 5-ds == dt)
}
