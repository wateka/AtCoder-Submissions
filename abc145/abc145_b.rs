#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    if n % 2 != 0 {
        println!("No");
        return;
    }

    for i in 0..(n/2) {
        if s[i] != s[n/2 + i] {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
