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

fn lt(p: &Vec<usize>, q: &Vec<usize>) -> bool {
    for i in 0..3 {
        if p[i] >= q[i] {
            return false;
        }
    }
    true
}

#[fastout]
fn main() {
    input! {
        p: [usize; 3],
        q: [usize; 3],
        u: [usize; 3],
        v: [usize; 3],
    }

    let ans = lt(&p, &v) && lt(&u, &q);;
    yes_no!(ans);
}
