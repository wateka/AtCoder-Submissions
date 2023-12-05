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
        mut abc: [usize; 3],
    }

    abc.sort();
    abc.reverse();
    let a = abc[0];
    let b = abc[1];
    let c = abc[2];

    let mut ans = 10000;

    for i in 0..10000 {
        if a*i > n {
            break;
        }

        for j in 0..(10000-i) {
            let x = a*i + b*j;

            if x > n {
                break;
            }

            if (n-x) % c == 0 {
                ans = ans.min(i + j + (n-x)/c);
            }
        }
    }

    putln!(ans);
}
