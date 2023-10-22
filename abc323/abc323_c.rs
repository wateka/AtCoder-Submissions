#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
use itertools::Itertools;
use proconio::{input, marker::Chars};
// use myoutput::{print_yes_no, putln};
// use mynibun::nibun;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; m],
        s: [Chars; n],
    }

    let scores = s.iter().enumerate().map(|(i, si)| {
        let mut score = (i as i64)+1;
        for (j, sij) in si.iter().enumerate() {
            if *sij == 'o' {
                score += a[j];
            }
        }
        score
    }).collect_vec();

    let score_max = scores.iter().max().unwrap();

    for (i, score) in scores.iter().enumerate() {
        let mut unsolved = vec![];
        for j in 0..m {
            if s[i][j] == 'x' {
                unsolved.push(a[j]);
            }
        }
        unsolved.sort();

        let mut rest = score_max - score;
        let mut need_solve_count = 0;
        while rest > 0 {
            rest -= unsolved.pop().unwrap();
            need_solve_count += 1;
        }

        println!("{}", need_solve_count);
    }
}
