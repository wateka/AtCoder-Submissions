#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use ac_library::{dsu::Dsu, modint::ModInt998244353 as Mint9, modint::Mod1000000007 as Mint10};
use itertools::{Itertools, izip, iproduct};
use superslice::Ext;
use proconio::{fastout, input, marker::{Chars, Usize1}};
macro_rules! yes_no { ($condition: expr) => {println!("{}", if $condition {"Yes"} else {"No"})} }
macro_rules! putln { ($value: expr) => {println!("{}", $value)} }

fn gcd(n: usize, m: usize) -> usize {
    if m == 0 {
        n
    } else {
        gcd(m, n % m)
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }

    let g = n * m / gcd(n, m);

    let rep_size = (g/n-1) + (g/m-1);
    let q = (k-1) / rep_size;
    let r = (k-1) % rep_size;

    let mut i = 1;
    let mut j = 1;
    let mut last = 0;
    for _ in 0..=r {
        if n*i < m*j {
            last = n*i;
            i += 1;
        } else {
            last = m*j;
            j += 1;
        }
    }

    let ans = (g * q) + last;
    putln!(ans);
}
