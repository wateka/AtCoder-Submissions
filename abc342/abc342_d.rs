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

fn div_by_square(x: usize) -> usize {
    let mut x = x;
    let mut i = 2;
    while (i*i) <= x {
        if x % (i*i) == 0 {
            x /= i*i;
            i = 2;
            continue;
        }
        i += 1;
    }
    x
}

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let b = a.into_iter().map(div_by_square);

    let mut map = HashMap::new();
    let mut count0 = 0;
    for k in b {
        if k == 0 {
            count0 += 1;
            continue;
        }
        let v = map.get(&k).unwrap_or(&0usize);
        map.insert(k, v+1);
    }

    let mut res = 0;

    if count0 > 0 {
        res += count0 * (n-1) - (count0 * (count0-1) / 2);
    }

    for (_, v) in map {
        res += v * (v-1) / 2;
    }

    putln!(res);
}
