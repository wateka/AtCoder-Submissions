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

fn dfs(u: usize, edges: &Vec<Vec<usize>>, seen: &mut Vec<Option<bool>>, b: bool) {
    if seen[u].is_some() {
        return
    }
    seen[u] = Some(b);
    for v in &edges[u] {
        dfs(*v, edges, seen, !b);
    }
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

    let mut colors = vec![None; n];
    dfs(0, &edges, &mut colors, true);

    let filter_b = colors
        .iter()
        .filter(|c| c.unwrap())
        .count() > n/2;
    
    let ans = (1..=n)
        .filter(|i| colors[i-1].unwrap() == filter_b)
        .take(n/2)
        .join(" ");
    
    putln!(ans)
}
