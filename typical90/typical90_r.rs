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

const D_PI: f64 = std::f64::consts::PI * 2.0;

fn theta(t: f64, l: f64, x: f64, y: f64, e: f64) -> f64 {
    // let s = D_PI * e / t;
    // let dx = x;
    // let dy = y - l/2.0 * s.sin();
    // let dz = -l/2.0 * s.cos() + l/2.0;

    let s = -D_PI * e / t;
    let dx = x;
    let dy = y - l/2.0 * s.sin();
    let dz = -l/2.0 * s.cos() + l/2.0;

    let m1 = (dx*dx + dy*dy).sqrt();
    let m2 = (dx*dx + dy*dy + dz*dz).sqrt();
    return 360.0 / D_PI * (m1/m2).acos()
}

#[fastout]
fn main() {
    input! {
        t: f64,
        l: f64,
        x: f64,
        y: f64,
        q: usize,
        e: [f64; q],
    }

    for ei in e {
        putln!(theta(t, l, x, y, ei))
    }
}
