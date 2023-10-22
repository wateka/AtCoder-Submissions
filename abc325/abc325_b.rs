#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
use proconio::{input, marker::Chars};
// use myoutput::{print_yes_no, putln};
// use mynibun::nibun;

fn main() {
    input! {
        n: usize,
        wx: [(i64, usize); n],
    }

    let mut ans = vec![0; 24];

    for (wi, xi) in wx {
        for i in 9..18 {
            ans[(xi+i)%24] += wi;
        }
    }

    println!("{}", ans.iter().max().unwrap());
}
