#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]

use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet, vec_deque, VecDeque};
use ac_library::{dsu::Dsu, modint::ModInt998244353 as Mint9, modint::Mod1000000007 as Mint10};
use itertools::{Itertools, izip, iproduct};
use superslice::Ext;
use proconio::{fastout, input, marker::{Chars, Usize1}};
macro_rules! yes_no { ($condition: expr) => {println!("{}", if $condition {"Yes"} else {"No"})} }
macro_rules! putln { ($value: expr) => {println!("{}", $value)} }

#[fastout]
fn main() {
    input! {
        n: i64,
        q: usize,
    }

    let mut v = VecDeque::new();
    for i in 0..n {
        v.push_back((i+1i64, 0i64));
    }

    for _ in 0..q {
        input! {t: usize}
        if t == 1 {
            input! {c: char}
            let (prev_x, prev_y) = v[0];
            let head = if c == 'R' {
                (prev_x+1, prev_y)
            } else if c == 'L' {
                (prev_x-1, prev_y)
            } else if c == 'U' {
                (prev_x, prev_y+1)
            } else {
                (prev_x, prev_y-1)
            };
            v.push_front(head);
            v.pop_back();
        } else {
            input! {p: Usize1}
            let (x, y) = v[p];
            println!("{} {}", x, y);
        }
    }
}
