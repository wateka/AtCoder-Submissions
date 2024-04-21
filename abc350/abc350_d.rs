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
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let mut dsu = Dsu::new(n);
    let mut friend_counts = vec![0; n];
    for (a, b) in ab {
        friend_counts[a] += 1;
        dsu.merge(a, b);
    }

    let mut sum = 0;
    let groups = dsu.groups();
    for g in groups {
        let g_len = g.len();
        let f_len = g.iter().map(|&j| friend_counts[j]).sum::<usize>();
        sum += g_len*(g_len-1)/2 - f_len;
    }

    putln!(sum);
}
