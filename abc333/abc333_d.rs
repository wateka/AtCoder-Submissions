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

fn dfs(u: usize, edges: &Vec<Vec<usize>>, seen: &mut Vec<bool>) -> usize {
    if seen[u] {
        return 0;
    }
    seen[u] = true;

    eprint!("{} ", u);

    let mut ret = 1;
    for v in edges[u].iter() {
        ret += dfs(*v, edges, seen);
    }
    ret
}

#[fastout]
fn main() {
    input! {
        n: usize,
        uvs: [(Usize1, Usize1); n-1],
    }

    let mut edges = vec![vec![]; n];
    for (u, v) in uvs {
        edges[u].push(v);
        edges[v].push(u);
    }

    if edges[0].len() == 1 {
        putln!(1);
        return;
    }

    let mut seen = vec![false; n];
    seen[0] = true;

    let mut costs = vec![0; edges[0].len()];
    for i in 0..edges[0].len() {
        let a = dfs(edges[0][i], &edges, &mut seen);
        costs[i] = a;
        eprintln!()
    }

    let mut ans = 100000000;
    let costs_sum = costs.iter().sum::<usize>();
    for cost in costs {
        ans = ans.min(costs_sum - cost);
    }

    for i in 0..edges.len() {
        eprintln!("{} -> {}", i, edges[i].iter().join(" "))
    }

    putln!(ans + 1);
}
