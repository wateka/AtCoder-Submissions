#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut count = 0;
    let mut st = Vec::new();
    for si in s {
        if st.len() > 0 && st[st.len()-1] != si {
            st.pop();
            count += 2;
        } else {
            st.push(si);
        }
    }

    println!("{}", count);
}
