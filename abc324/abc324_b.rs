#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
use proconio::{input, marker::Chars};
// use myoutput::{print_yes_no, putln};
// use mynibun::nibun;

fn main() {
    input! {
        mut n: i64
    }

    loop {
        if n % 2 == 0 {
            n = n/2;
        } else if n % 3 == 0 {
            n = n/3;
        } else {
            break;
        }
    }

    if n == 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
