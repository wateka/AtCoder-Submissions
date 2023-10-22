#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
use ac_library::Dsu;
use proconio::{input, marker::Chars};
// use myoutput::{print_yes_no, putln};
// use mynibun::nibun;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut s: [Chars; h],
    }

    let mut s_num = vec![vec![None; w]; h];
    let mut hash_count = 0;
    for (i, si) in s.iter().enumerate() {
        for (j, sij) in si.iter().enumerate() {
            if *sij == '#' {
                s_num[i][j] = Some(hash_count);
                hash_count += 1;
            } 
        }
    }

    let mut dsu = Dsu::new(hash_count);

    for i in 0..h {
        for j in 0..w {
            let Some(sij) = s_num[i][j] else { continue; };

            if i > 0 && s_num[i-1][j].is_some() {
                dsu.merge(sij, s_num[i-1][j].unwrap());
            }

            if j > 0 && s_num[i][j-1].is_some() {
                dsu.merge(sij, s_num[i][j-1].unwrap());
            }

            if i > 0 && j > 0 && s_num[i-1][j-1].is_some() {
                dsu.merge(sij, s_num[i-1][j-1].unwrap());
            }

            if i > 0 && j < w-1 && s_num[i-1][j+1].is_some() {
                dsu.merge(sij, s_num[i-1][j+1].unwrap());
            }
        }
    }

    println!("{}", dsu.groups().len());
}
