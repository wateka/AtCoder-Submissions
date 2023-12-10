#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]

use std::{collections::{HashMap, HashSet, BTreeMap, BTreeSet}, mem::swap};
use ac_library::{dsu::Dsu, modint::ModInt998244353 as Mint9, modint::Mod1000000007 as Mint10};
use itertools::{Itertools, izip, iproduct};
use superslice::Ext;
use proconio::{fastout, input, marker::{Chars, Usize1}};
macro_rules! yes_no { ($condition: expr) => {println!("{}", if $condition {"Yes"} else {"No"})} }
macro_rules! putln { ($value: expr) => {println!("{}", $value)} }

fn bubble(p: &Vec<usize>) -> usize {
    let mut p = p.clone();
    let len = p.len();
    let mut t = 0;
    let mut sorted = false;
    while !sorted {
        sorted = true;
        for i in 0..(len-1) {
            if p[i] > p[i+1] {
                sorted = false;
                t += 1;
                let temp = p[i];
                p[i] = p[i+1];
                p[i+1] = temp;
            }
        }
    }
    t
}

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[i64; w]; h],
        b: [[i64; w]; h],
    }

    let mut ans = usize::MAX;
    for p in (0..h).permutations(h) {
        'q: for q in (0..w).permutations(w) {
            for i in 0..h {
                for j in 0..w {
                    let pi = p[i];
                    let qj = q[j];
                    if a[pi][qj] != b[i][j] {
                        continue 'q;
                    }
                }
            }
            ans = ans.min(bubble(&p) + bubble(&q));
        }
    }
    if ans == usize::MAX {
        putln!(-1);
        return;
    }
    putln!(ans)
}
