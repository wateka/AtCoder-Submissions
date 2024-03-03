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
        t: usize,
        ab: [(Usize1, i64); t],
    }

    let mut scores = vec![0i64; n];
    let mut map = HashMap::new();
    let mut set = HashSet::new();

    map.insert(0, n);
    set.insert(0);

    for (a, b) in ab {
        let score = scores[a];
        let new_score = scores[a] + b;
        scores[a] = new_score;

        if let Some(&count) = map.get(&score) {
            if count == 1 {
                map.remove(&score);
                set.remove(&score);
            } else {
                map.insert(score, count-1);
            }
        }

        let &new_count = map.get(&new_score).unwrap_or(&0);
        map.insert(new_score, new_count + 1);
        set.insert(new_score);

        putln!(set.len());
    }
}
