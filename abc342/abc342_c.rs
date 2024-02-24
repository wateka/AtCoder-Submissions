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
        s: Chars,
        q: usize,
        cd: [(char, char); q],
    }

    let mut table = (0usize..26).into_iter().map(|i| (i as u8 + 'a' as u8) as char).collect_vec();

    for (c, d) in cd {
        for i in 0..26 {
            if table[i] == c {
                table[i] = d;
            }
        }
    }

    for c in s {
        let i = (c as u8 - 'a' as u8) as usize;
        print!("{}", table[i]);
    }
    println!();
}
