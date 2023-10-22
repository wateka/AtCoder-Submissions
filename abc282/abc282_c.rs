#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        s: Chars,
    }

    let mut inside = false;
    for si in s {
        if si == '"' {
            inside = !inside;
        }

        if !inside && si == ',' {
            print!(".");
            continue;
        }

        print!("{}", si);
    }

    println!();
}
