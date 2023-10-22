#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }

    for p in (0..n).permutations(n) {
        let mut ok = true;
        for w in p.windows(2) {
            let (i, i_next) = (w[0], w[1]);
            let mut c = 0;
            for j in 0..m {
                if s[i][j] != s[i_next][j] {
                    c += 1;
                }
            }

            if c != 1 {
                ok = false;
                break;
            }
        }

        if ok {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
