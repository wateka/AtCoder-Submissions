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

fn count(y: usize, x: usize, n: usize, acc: &Vec<Vec<usize>>) -> usize {
    let rx = x / n;
    let ry = y / n;

    return acc[n][n] * rx * ry
        + acc[y%n][n] * rx
        + acc[n][x%n] * ry
        + acc[y%n][x%n];
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        p: [Chars; n],
        queries: [(usize,usize,usize,usize); q],
    }

    let p = p.into_iter().map(|pi| {
        pi.into_iter().map(|pij| {
            if pij == 'W' {
                0usize
            } else {
                1usize
            }
        }).collect_vec()
    }).collect_vec();

    let mut p_acc = vec![vec![0usize; n+1]; n+1];
    for i in 1..=n {
        for j in 1..=n {
            p_acc[i][j] = p_acc[i-1][j] + p_acc[i][j-1] - p_acc[i-1][j-1] + p[i-1][j-1];
        }
    }

    for (a, b, c, d) in queries {
        let (a, b, c, d) = (a, b, c+1, d+1);
        let ans =
            count(c, d, n, &p_acc)
            + count(a, b, n, &p_acc)
            - count(a, d, n, &p_acc)
            - count(c, b, n, &p_acc);
        putln!(ans)
    }
}
