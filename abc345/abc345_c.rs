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
        s: Chars
    }

    let n = s.len();
    let mut counts = vec![0; 26];
    for c in s {
        let ci = (c as u8 - 'a' as u8) as usize;
        counts[ci] += 1;
    }

    let mut ans = n * (n-1) / 2;
    let mut dup = false;
    for count in counts {
        if count >= 2 {
            dup = true;
            ans -= count*(count-1)/2;
        }
    }

    if dup {
        ans += 1;
    }

    putln!(ans);
}
