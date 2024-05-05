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
    }

    let mut c = vec![0; m];
    let mut a = vec![vec![]; m];

    for i in 0..m {
        input! {
            ki: usize,
            ci: usize,
            ai: [Usize1; ki],
        }
        c[i] = ci;
        for aij in ai {
            a[i].push(aij);
        }
    }

    let order = (0..m).sorted_by_key(|&i| c[i]);

    let mut cost = 0;
    let mut dsu = Dsu::new(n);

    
    for i in order {
        let ci = c[i];
        let ai = &a[i];

        let mut g = 0;
        for j in 1..ai.len() {
            if !dsu.same(ai[0], ai[j]) {
                g += 1;
                dsu.merge(ai[0], ai[j]);
            }
        }

        cost += g * ci;
        // eprintln!("groups:{}->{}, ci:{}, total_cost:{}", prev_g, g, ci, cost);
    }

    if dsu.groups().len() > 1 {
        putln!(-1);
    } else {
        putln!(cost);
    }
}
