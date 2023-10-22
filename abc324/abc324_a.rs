#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
use proconio::{input, marker::Chars};
// use myoutput::{print_yes_no, putln};
// use mynibun::nibun;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let a0 = a[0];

    for ai in a {
        if ai != a0 {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
