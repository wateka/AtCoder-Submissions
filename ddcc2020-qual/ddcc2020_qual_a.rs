#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
use proconio::{input, marker::{Chars, Usize1}};

fn main() {
    input! {
        x: Usize1,
        y: Usize1,
    }

    let money = [300000, 200000, 100000];

    let mut ans = 0;
    
    if x < 3 {
        ans += money[x]
    }
    if y < 3 {
        ans += money[y]
    }
    if x == 0 && y == 0 {
        ans += 400000;
    }

    println!("{}", ans);
}
