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
        a: [i64; n],
    }

    let mut a_sorted = a.iter().cloned().collect_vec();
    a_sorted.sort();
    // eprintln!("{:?}", a_sorted);

    let mut a_sorted_acc = vec![0; n+1];
    for i in 0..n {
        a_sorted_acc[i+1] = a_sorted_acc[i] + a_sorted[i];
    }
    // eprintln!("{:?}", a_sorted_acc);

    let mut b = vec![0i64; n];

    for i in 0..n {
        let j = a_sorted.upper_bound(&a[i]);
        // eprintln!("{} {} ", a[i], j);
        b[i] = a_sorted_acc[n] - a_sorted_acc[j];
    }
    
    putln!(b.iter().join(" "))
}
