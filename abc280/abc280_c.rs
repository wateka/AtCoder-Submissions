#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    for (i, (ti, si)) in t.iter().zip(s).enumerate() {
        if si != *ti {
            println!("{}", i+1);
            return;
        }
    }

    println!("{}", t.len());
}
