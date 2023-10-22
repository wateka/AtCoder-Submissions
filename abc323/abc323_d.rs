#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
use proconio::{input, marker::Chars};
// use myoutput::{print_yes_no, putln};
// use mynibun::nibun;

fn main() {
    input! {
        n: usize,
        sc: [(i64, usize); n],
    }

    let mut sc = sc.into_iter().collect::<BTreeMap<i64, usize>>();

    let mut ans = 0;
    while !sc.is_empty() {
        let (&sm_si, &sm_ci) = sc.iter().next().unwrap();
        let lg_si = sm_si * 2;
        if sm_ci > 1 {
            let lg_ci = sc.get(&lg_si).unwrap_or(&0) + (sm_ci/2);
            sc.insert(lg_si, lg_ci);
        }
        sc.remove(&sm_si);
        ans += sm_ci % 2;
    }

    println!("{}", ans);
}
