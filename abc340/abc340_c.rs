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

fn div(n: u128, memo: &mut HashMap<u128, u128>) -> u128 {
    if n <= 1 {
        return 0;
    }
    
    if let Some(&sum) = memo.get(&n) {
        return sum
    }

    let a = div(n/2, memo);
    let b = div(n/2 + n%2, memo);
    let sum = n + a + b;
    memo.insert(n, sum);

    return sum;
}

#[fastout]
fn main() {
    input! {
        n: u128
    }

    let mut memo = HashMap::new();
    let ans = div(n, &mut memo);

    println!("{}", ans);
}
