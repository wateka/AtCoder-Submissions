#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]

use std::{collections::{BTreeMap, BTreeSet, HashMap, HashSet}, vec};
use ac_library::{dsu::Dsu, modint::{Mod1000000007 as Mint10, ModInt998244353 as Mint9}, Additive, Segtree};
use itertools::{Itertools, izip, iproduct};
use superslice::Ext;
use proconio::{fastout, input, marker::{Chars, Usize1}};
macro_rules! yes_no { ($condition: expr) => {println!("{}", if $condition {"Yes"} else {"No"})} }
macro_rules! putln { ($value: expr) => {println!("{}", $value)} }

fn rev_seg_rightside(seg: &mut Segtree::<Additive<usize>>, p: usize) {
    let x = seg.get(p);
    seg.set(p, (x+1)%2);
}

fn rev_seg(seg: &mut Segtree::<Additive<usize>>, l: usize, r: usize, n: usize) {
    if l > 0 {
        rev_seg_rightside(seg, l-1);
    }
    if r < n-1 {
        rev_seg_rightside(seg, r);
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        queries: [(usize, Usize1, Usize1); q],
    }

    let mut seg = Segtree::<Additive<usize>>::new(n-1);
    for i in 0..(n-1) {
        if s[i] != s[i+1] {
            seg.set(i, 1);
        }
    }

    for (t, l, r) in queries {
        if t == 1 {
            rev_seg(&mut seg, l, r, n);
        } else {
            let ok = seg.prod(l..r) == r-l;
            if ok {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
