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

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut v = vec![vec![0usize; n]; n];

    let mut x = 0;
    let mut y = 0;
    let mut m = 0;
    for i in 1..(n*n) {
        v[x][y] = i;
        if m == 0 {
            if x < n-1 && v[x+1][y] == 0 {
                x += 1;
            } else {
                m = 1;
                y += 1;
            }
        } else if m == 1 {
            if y < n-1 && v[x][y+1] == 0 {
                y += 1;
            } else {
                m = 2;
                x -= 1;
            }
        } else if m == 2 {
            if 0 < x && v[x-1][y] == 0 {
                x -= 1;
            } else {
                m = 3;
                y -= 1;
            }
        } else if m == 3 {
            if 0 < y && v[x][y-1] == 0 {
                y -= 1;
            } else {
                m = 0;
                x += 1;
            }
        }
    }

    for vi in v {
        for vij in vi {
            if vij == 0 {
                print!("T ");
            } else {
                print!("{} ", vij)
            }
        }
        println!();
    }
}
