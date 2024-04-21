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
        mut a: [Usize1; n],
    }

    let mut where_is = vec![0; n];
    for (i, &ai) in a.iter().enumerate() {
        where_is[ai] = i;
    }
    let mut history = vec![];

    for i in 0..n {
        let ai = a[i];
        if ai != i {
            let pi = where_is[i];
            a[pi] = ai;
            a[i] = i;
            where_is[i] = i;
            where_is[ai] = pi;
            history.push([i+1, pi+1]);
            // eprintln!("{:?}",a);
        }
    }

    putln!(history.len());
    for h in history {
        putln!(h.iter().join(" "));
    }
}
