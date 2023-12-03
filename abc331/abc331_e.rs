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
        l: usize,
        a: [i64; n],
        b: [i64; m],
        cd: [(Usize1, Usize1); l],
    }

    let mut b_sort_index = (0..m).collect_vec();
    b_sort_index.sort_by_key(|&i| b[i]);
    b_sort_index.reverse();

    let cd = cd.into_iter().collect::<HashSet<_>>();
    let ans = a.into_iter().enumerate().map(|(i, ai)| {
        for &j in b_sort_index.iter() {
            if !cd.contains(&(i, j)) {
                //eprintln!("A_{}={}, B_{}={}", i, ai, j, b[j]);
                return ai + b[j];
            }
        }
        return 0;
    }).max().unwrap();

    putln!(ans);
}
