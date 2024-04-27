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

    let mut s = vec![];
    for i in 0..n {
        let ai= a[i];
        s.push(ai);
        while s.len() > 1 && s[s.len()-2] == s[s.len()-1] {
            let s1 = s.pop().unwrap();
            s.pop();
            s.push(s1+1);
        }
    }

    putln!(s.len());
}
