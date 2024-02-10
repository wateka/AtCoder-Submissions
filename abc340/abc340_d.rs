#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]

use std::{cmp::Reverse, collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque}};
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
        abx: [(i64, i64, Usize1); n-1],
    }

    let mut edges = vec![vec![]; n];
    for (i, &(ai, bi, xi)) in abx.iter().enumerate() {
        edges[i].push((i+1, ai));
        edges[i].push((xi, bi));
    }

    let mut d = vec![i64::MAX; n];
    d[0] = 0;
    
    let mut heap = BinaryHeap::new();
    for (i, &di) in d.iter().enumerate() {
        heap.push((Reverse(di), i));
    }

    while !heap.is_empty() {
        let (Reverse(d1), u) = heap.pop().unwrap();
        if d[u] < d1 {
            continue;
        }
        for &(v, d2) in edges[u].iter() {
            let alt = d1 + d2;
            if d[v] > alt {
                d[v] = alt;
                heap.push((Reverse(alt), v));
            }
        }
    }

    println!("{}", d[n-1]);
}
