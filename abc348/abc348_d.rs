#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]

use std::collections::{vec_deque, BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use ac_library::{dsu::Dsu, modint::ModInt998244353 as Mint9, modint::Mod1000000007 as Mint10};
use itertools::{Itertools, izip, iproduct};
use superslice::Ext;
use proconio::{fastout, input, marker::{Chars, Usize1}};
macro_rules! yes_no { ($condition: expr) => {println!("{}", if $condition {"Yes"} else {"No"})} }
macro_rules! putln { ($value: expr) => {println!("{}", $value)} }

fn dfs(u: usize, edges: &Vec<Vec<usize>>, seen: &mut Vec<bool>, goal: usize) -> bool {
    if seen[u] {
        return false;
    }
    seen[u] = true;
    if u == goal {
        return true;
    }
    return edges[u].iter().any(|v| dfs(*v, edges, seen, goal));
}

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
        n: usize,
        rce: [(Usize1, Usize1, i64); n],
    }

    let s = iproduct!(0..h, 0..w).find(|&(i, j)| a[i][j] == 'S').unwrap();
    let t = iproduct!(0..h, 0..w).find(|&(i, j)| a[i][j] == 'T').unwrap();

    let mut ds = vec![vec![vec![false; w]; h]; n];

    for i in 0..n {
        let mut queue = VecDeque::new();
        queue.push_back(rce[i]);
        while !queue.is_empty() {
            let (r, c, e) = queue.pop_front().unwrap();
            if ds[i][r][c] {
                continue;
            }
            ds[i][r][c] = true;
            if e == 0 {
                continue;
            }
            if r > 0 && a[r-1][c] != '#' {
                queue.push_back((r-1, c, e-1));
            }
            if r < h-1 && a[r+1][c] != '#' {
                queue.push_back((r+1, c, e-1));
            }
            if c > 0 && a[r][c-1] != '#' {
                queue.push_back((r, c-1, e-1));
            }
            if c < w-1 && a[r][c+1] != '#' {
                queue.push_back((r, c+1, e-1));
            }
        }
    }

    let mut ps = rce.iter().map(|&(ri, ci, ei)| (ri, ci)).collect_vec();
    ps.push(t);

    let mut si = None;
    for (i, p) in ps.iter().enumerate() {
        if *p == s {
            si = Some(i);
        }
    }
    if si.is_none() {
        println!("No");
        return;
    }

    let mut edges = vec![vec![]; n];
    for i in 0..n {
        for j in 0..=n {
            if i == j {
                continue;
            }
            let (rj, cj) = ps[j];
            if ds[i][rj][cj] {
                edges[i].push(j)
            }
        }
    }

    let mut seen = vec![false; n+1];
    
    let ans = dfs(si.unwrap(), &edges, &mut seen, n);
    yes_no!(ans);
}
