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

fn to_num(c: char) -> usize {
    (c as u8 - '0' as u8) as usize
}

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let mut res = false;
    if s[0] == 'A' && s[1] == 'B' && s[2] == 'C' {
        let n = to_num(s[3])*100 + to_num(s[4])*10 + to_num(s[5]);
        res = n > 0 && n < 350 && n != 316;
    }

    yes_no!(res);
}
