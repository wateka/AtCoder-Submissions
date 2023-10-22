#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let mut count = 0;
    let mut skip_next = false;
    for w in s.windows(2) {
        let (si, si_next) = (w[0], w[1]);
        if skip_next {
            skip_next = false;
            continue;
        }

        if si == '0' && si_next == '0' {
            skip_next = true;
        }

        count += 1;
    }

    if !skip_next {
        count += 1;
    }

    println!("{}", count);
}
