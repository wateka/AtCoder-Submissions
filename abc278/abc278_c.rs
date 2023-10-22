#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
use proconio::{input, marker::{Chars, Usize1}};

fn main() {
    input! {
        n: usize,
        q: usize,
        queries: [(usize, Usize1, Usize1); q],
    }

    let mut follows = BTreeSet::new();

    for (ti, ai, bi) in queries {
        match ti {
            1 => {
                follows.insert((ai, bi));
            },
            2 => {
                follows.remove(&(ai, bi));
            },
            3 => {
                if follows.contains(&(ai, bi)) && follows.contains(&(bi, ai)) {
                    println!("Yes")
                } else {
                    println!("No");
                }
            },
            _ => panic!(),
        }
    }
}
