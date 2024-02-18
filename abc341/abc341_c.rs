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
        n: usize,
        t: Chars,
        s: [Chars; h],
    }

    let mut res = 0;

    for y in 1..(h-1) {
        'l: for x in 1..(w-1) {
            if s[y][x] == '#' {
                continue 'l;
            }
            let mut i = y;
            let mut j = x;
            for &ti in t.iter() {
                match ti {
                    'L' => j -= 1 ,
                    'R' => j += 1 ,
                    'U' => i -= 1 ,
                    'D' => i += 1 ,
                    _ => unreachable!(),
                }
                if s[i][j] == '#' {
                    continue 'l;
                }
            }
            res += 1;
        }
    }

    putln!(res);
}
