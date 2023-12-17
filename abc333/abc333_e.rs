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
        tx: [(usize, Usize1); n]
    }

    let mut used = vec![];
    let mut remains = vec![vec![]; n];

    let mut i = 0;
    for &(t, x) in tx.iter() {
        if t == 1 {
            remains[x].push(i);
            used.push(0);
            i += 1;
            // eprintln!("added portion {}", x);
        } else {
            if let Some(j) = remains[x].pop() {
                used[j] = 1;
            } else {
                // eprintln!("no portion {}", x);
                println!("-1");
                return
            };
        }
    }

    let mut count = 0;
    let mut k = 0;
    let mut i = 0;
    for &(t, _) in tx.iter() {
        if t == 1{
            if used[i] == 1 {
                count += 1;
                k = k.max(count);
            }
            i += 1;
        } else {
            count -= 1;
        }
    }

    putln!(k);
    putln!(used.iter().join(" "))
}
