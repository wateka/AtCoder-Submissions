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
        s: Chars,
        t: Chars,
    }

    let t = t.into_iter().map(|ti| (ti as u8 + 'a' as u8 - 'A' as u8) as char).collect_vec();

    let mut j = 0;
    for si in s {
        let tj = t[j];
        if si == tj {
            j += 1;
        }
        if j >= 3 {
            break;
        }
    }

    let ans = j == 3 || (j == 2 && t[2] == 'x');
    yes_no!(ans);
}
