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
        a: usize,
        b: usize,
        d: [usize; n],
    }

    let l = a+b;

    let youbi: BTreeSet<_> = d.into_iter().map(|di| di % l).collect();
    let mut youbi = youbi.into_iter().collect_vec();
    youbi.push(youbi[0] + a+b);

    for (i, j) in youbi.iter().tuple_windows() {
        if j - i > b {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
