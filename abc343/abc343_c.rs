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
        n: usize
    }

    let mut res = 0;
    'outer: for l in 0..1000000 {
        let l3 = l*l*l;
        if l3 > n {
            break;
        }
        let s = l3.to_string().into_bytes();
        for i in 0..(s.len()/2) {
            if s[i] != s[s.len()-1-i] {
                continue 'outer;
            }
        }
        res = l3;
    }
    putln!(res);
}
