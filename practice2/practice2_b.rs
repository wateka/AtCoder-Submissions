#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]

use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
use ac_library::{dsu::Dsu, modint::{Mod1000000007 as Mint10, ModInt998244353 as Mint9}, FenwickTree};
use itertools::{Itertools, izip, iproduct};
use superslice::Ext;
use proconio::{fastout, input, marker::{Chars, Usize1}};
macro_rules! yes_no { ($condition: expr) => {println!("{}", if $condition {"Yes"} else {"No"})} }
macro_rules! putln { ($value: expr) => {println!("{}", $value)} }

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        queries: [(usize, usize, usize); q],
    }

    let mut fw = FenwickTree::new(n, 0);
    a.into_iter().enumerate().for_each(|(i, ai)| fw.add(i, ai));
    for query in queries {
        if query.0 == 0 {
            let (_, p, x) = query;
            fw.add(p, x);
        } else {
            let (_, l, r) = query;
            let ans = fw.sum(l..r);
            putln!(ans);
        }
    }
}
