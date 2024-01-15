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
        a: [usize; n],
    }

    let mut left = vec![0; n];
    let mut right = vec![0; n];

    left[0] = 1;
    right[n-1] = 1;
    for i in 1..n {
        left[i] = a[i].min(left[i-1]+1);
        right[n-i-1] = a[n-i-1].min(right[n-i]+1);
    }

    let mut ans = 0;
    for i in 0..n {
        let h = left[i].min(right[i]);
        ans = ans.max(h);
    }

    putln!(ans);
}
