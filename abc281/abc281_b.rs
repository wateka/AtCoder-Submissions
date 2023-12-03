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

fn is_number(c: char) -> bool {
    (('0' as u8)..=('9' as u8)).contains(&(c as u8))
}

fn is_upper(c: char) -> bool {
    (('A' as u8)..=('Z' as u8)).contains(&(c as u8))
}

#[fastout]
fn main() {
    input! {
        s: Chars
    }

    yes_no!(
        s.len() == 8
        && is_upper(s[0])
        && is_number(s[1])
        && is_number(s[2])
        && is_number(s[3])
        && is_number(s[4])
        && is_number(s[5])
        && is_number(s[6])
        && is_upper(s[7])
        && s[1] != '0'
    )
}
