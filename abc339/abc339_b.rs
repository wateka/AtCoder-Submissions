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
        h: i64,
        w: i64,
        n: usize,
    }

    let uh = h as usize;
    let uw = w as usize;

    let (mut x, mut y) = (0i64, 0i64);
    let (mut dx, mut dy) = (0i64, -1i64);
    let mut drawn = vec![vec![false; uw]; uh];

    for _ in 0..n {
        let ux = x as usize;
        let uy = y as usize;
        if drawn[uy][ux] {
            (dx, dy) = (dy, -dx);
        } else {
            (dx, dy) = (-dy, dx);
        }
        drawn[uy][ux] = !drawn[uy][ux];
        x = (x + dx + w) % w;
        y = (y + dy + h) % h;
    }

    for line in drawn {
        let res: String = line.into_iter()
            .map(|b| if b { '#' } else { '.' })
            .join("");
        putln!(res);
    }
}
