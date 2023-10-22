#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
use proconio::{input, marker::Chars};
// use myoutput::{print_yes_no, putln};
// use mynibun::nibun;

fn main() {
    input! {
        s: Chars
    }

    let mut f = true;
    for (i, si) in s.into_iter().enumerate() {
        if i % 2 == 1 && si == '1' {
            f = false;
        }
    }

    println!("{}", if f {"Yes"} else {"No"});
}
