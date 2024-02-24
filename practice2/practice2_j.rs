#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]

use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
use ac_library::{dsu::Dsu, modint::{Mod1000000007 as Mint10, ModInt998244353 as Mint9}, Max, Segtree};
use itertools::{Itertools, izip, iproduct};
use superslice::Ext;
use proconio::{fastout, input, marker::{Chars, Usize1}};
macro_rules! yes_no { ($condition: expr) => {println!("{}", if $condition {"Yes"} else {"No"})} }
macro_rules! putln { ($value: expr) => {println!("{}", $value)} }

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        queries: [(usize, usize, usize); q],
    }

    let mut seg: Segtree<Max<usize>> = Segtree::new(n);
    a.into_iter().enumerate().for_each(|(p, x)| {
        seg.set(p, x);
    });

    for query in queries {
        let t = query.0;
        match t {
            1 => {
                let (_, x, v) = query;
                let x = x - 1;
                seg.set(x, v);
            },
            2 => {
                let (_, l, r) = query;
                let l = l - 1;
                let r = r - 1;
                let res = seg.prod(l..=r);
                putln!(res);
            },
            3 => {
                let (_, x, v) = query;
                let x = x - 1;
                let res = if v == 0 {
                    x
                } else {
                    seg.max_right(x, |&ai| ai < v)
                };
                putln!(res + 1);
            },
            _ => unreachable!(),
        }
    }
}
