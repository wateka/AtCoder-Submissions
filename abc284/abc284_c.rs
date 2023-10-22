#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
use ac_library::Dsu;
use proconio::{input, marker::{Chars, Usize1}};

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
    }

    let mut dsu = Dsu::new(n);
    for (u, v) in edges {
        dsu.merge(u, v);
    }

    println!("{}", dsu.groups().len());
    
}
