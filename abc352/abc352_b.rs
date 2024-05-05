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

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut s: VecDeque<_> = s.into();
    let mut ans = vec![];
    for (i, ti) in t.into_iter().enumerate() {
        if s[0] == ti {
            ans.push(i);
            s.pop_front();
        }
    }
    putln!(ans.into_iter().map(|i| i+1).join(" "));
}
