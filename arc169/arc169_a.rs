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

fn dfs(u: usize, edges: &Vec<Vec<usize>>, a: &Vec<i64>, depth: usize, sum_by_depth: &mut BTreeMap<usize, i64>) -> (i64, usize) {
    let sum_thisd = sum_by_depth.get(&depth).unwrap_or(&0);
    sum_by_depth.insert(depth, sum_thisd + a[u]);

    let mut max_depth = 0;
    let mut sum = 0;
    for v in &edges[u] {
        let (s, d) = dfs(*v, edges, a, depth+1, sum_by_depth);
        if d > max_depth {
            max_depth = d;
            sum = s;
        } else if d == max_depth {
            sum += s;
        }
    }

    if sum == 0 {
        return (a[u], depth);
    }
    
    return (sum, max_depth);
}

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
        p: [Usize1; n-1],
    }

    let mut edges = vec![vec![]; n];
    for i in 1..n {
        let pi = p[i-1];
        edges[pi].push(i);
    }

    let mut sum_by_depth = BTreeMap::new();
    let (a0_get, _depth) = dfs(0, &edges, &a, 0, &mut sum_by_depth);

    for (_d, sum) in sum_by_depth.into_iter().rev() {
        if sum > 0 {
            println!("+");
            return;
        } else if sum < 0 {
            println!("-");
            return;
        }
    }
    println!("0");
}
