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
        h: usize,
        w: usize,
        k: usize,
        s: [Chars; h],
    }

    let mut ok = false;
    let mut ans = usize::MAX;

    for i in 0..h {
        let mut cont = 0;
        let mut need = 0;
        for j in 0..w {
            let sij = s[i][j];
            if sij == 'x' {
                cont = 0;
                need = 0;
            } else {
                cont += 1;
                if sij == '.' {
                    need += 1;
                }
                if cont > k && s[i][j-k] == '.' {
                    need -= 1;
                }
                if cont >= k {
                    ok = true;
                    ans = ans.min(need);
                }
            }
        }
    }

    for j in 0..w {
        let mut cont = 0;
        let mut need = 0;
        for i in 0..h {
            let sij = s[i][j];
            if sij == 'x' {
                cont = 0;
                need = 0;
            } else {
                cont += 1;
                if sij == '.' {
                    need += 1;
                }
                if cont > k && s[i-k][j] == '.' {
                    need -= 1;
                }
                if cont >= k {
                    ok = true;
                    ans = ans.min(need);
                }
            }
        }
    }

    if ok {
        putln!(ans)
    } else {
        putln!(-1)
    }
}
