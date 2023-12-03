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
        mut s: Chars
    }

    s.push('\0');
    let mut cnt = 1;
    let mut cnts = vec![0; 26];
    for (sp, si) in s.into_iter().tuple_windows() {
        if sp == si {
            cnt += 1;
        } else {
            let j = sp as usize - 'a' as usize;
            cnts[j] = cnts[j].max(cnt);
            cnt = 1;
        }
    }
    let ans: i64 = cnts.into_iter().sum();
    putln!(ans)
}
