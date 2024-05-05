#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]

use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
use ac_library::{dsu::Dsu, modint::{Mod1000000007 as Mint10, ModInt998244353 as Mint9}, Max, Min, Segtree};
use itertools::{Itertools, izip, iproduct};
use superslice::Ext;
use proconio::{fastout, input, marker::{Chars, Usize1}};
macro_rules! yes_no { ($condition: expr) => {println!("{}", if $condition {"Yes"} else {"No"})} }
macro_rules! putln { ($value: expr) => {println!("{}", $value)} }

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize; n],
    }

    let mut q = (0..n).collect_vec();
    q.sort_by_key(|&qi| p[qi]);

    let mut seg_max = Segtree::<Max<usize>>::new(n);
    let mut seg_min = Segtree::<Min<usize>>::new(n);

    for i in 0..n {
        seg_min.set(i, q[i]);
        seg_max.set(i, q[i]);
    }

    let mut ans = usize::MAX;
    for j in 0..(n-k+1) {
        let l = j;
        let r = j + k;
        let min = seg_min.prod(l..r);
        let max = seg_max.prod(l..r);
        let x = max - min;
        ans = ans.min(x);
    }
    
    putln!(ans);
}
