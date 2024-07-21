#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]

use ac_library::{dsu::Dsu, modint::Mod1000000007 as Mint10, modint::ModInt998244353 as Mint9};
use itertools::{iproduct, izip, Itertools, Permutations};
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    vec,
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

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }

    let mut vec = vec![];

    for p in (0..n).permutations(n) {
        let st = p.iter().map(|pi| s[*pi]).collect::<Vec<_>>();
        vec.push(st);
    }

    vec.sort();
    vec.dedup();

    let mut ans = 0;
    for s in vec.iter() {
        let mut ok = true;
        for i in 0..=(n - k) {
            let mut flag = true;
            for j in 0..=(k / 2) {
                if s[i + j] != s[i + (k - j - 1)] {
                    flag = false;
                }
            }
            if flag {
                ok = false;
            }
        }
        if ok {
            ans += 1;
        }
    }

    putln!(ans);
}
