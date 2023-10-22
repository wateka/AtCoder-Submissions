#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let s = (0..m).map(|_| {
        input! {
            ci: usize,
            si: [usize; ci],
        }
        si.into_iter().collect::<HashSet<usize>>()
    }).collect_vec();

    let mut count = 0;
    for b in 1..(1<<m) {
        let mut include_all = true;
        for x in 1..=n {
            let mut includes_x = false;
            for i in 0..m {
                if (b & 1<<i) != 0 && s[i].contains(&x) {
                    includes_x = true;
                    break;
                }
            }

            if !includes_x {
                include_all = false;
                break;
            }
        }

        if include_all {
            count += 1;
        }
    }

    println!("{}", count);
}
