#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]

use ac_library::{dsu::Dsu, modint::Mod1000000007 as Mint10, modint::ModInt998244353 as Mint9};
use itertools::{iproduct, izip, Itertools};
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::{
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet},
};
use superslice::Ext;
macro_rules! yes_no {
    ($condition: expr) => {
        println!("{}", if $condition { "Yes" } else { "No" })
    };
}
macro_rules! putln {
    ($value: expr) => {
        println!("{}", $value)
    };
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
        uvb: [(Usize1, Usize1, i64); m],
    }

    let mut edges = vec![vec![]; n];

    for (u, v, b) in uvb {
        edges[u].push((v, a[v] + b));
        edges[v].push((u, a[u] + b));
    }

    let mut heap = BinaryHeap::new();
    let mut dist = vec![i64::MAX; n];
    dist[0] = 0;

    heap.push(Reverse((0, 0)));

    while !heap.is_empty() {
        let Reverse((d, u)) = heap.pop().unwrap();
        if d > dist[u] {
            continue;
        }

        for &(v, w) in edges[u].iter() {
            let new_d = dist[u] + w;
            if new_d < dist[v] {
                dist[v] = new_d;
                heap.push(Reverse((new_d, v)));
            }
        }
    }

    putln!(dist.iter().map(|d| d + a[0]).skip(1).join(" "));
}
