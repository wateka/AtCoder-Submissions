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
        s: Chars
    }

    let mut muji_clean = m;
    let mut need_max = 0;
    let mut need = 0;

    for c in s {
        if c == '0' {
            need = 0;
            muji_clean = m;
        } else if c == '1' {
            if muji_clean > 0 {
                muji_clean -= 1;
            } else {
                need += 1;
                need_max = need_max.max(need);
            }
        } else {
            need += 1;
            need_max = need_max.max(need);
        }
    }
    putln!(need_max)
}
