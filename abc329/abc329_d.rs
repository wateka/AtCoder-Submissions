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
        m: usize,
        a: [Usize1; m],
    }

    let mut max_ai = 0;
    let mut max = 0;
    let mut cnts = vec![0; n];
    for ai in a {
        cnts[ai] += 1;
        let cnt = cnts[ai];
        if cnt > max {
            max = cnt;
            max_ai = ai;
        } else if cnt == max && ai < max_ai {
            max_ai = ai;
        }

        putln!(max_ai+1)
    }
}
