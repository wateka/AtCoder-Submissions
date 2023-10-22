#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
use proconio::{input, marker::Chars};
use itertools::Itertools;
// use myoutput::{print_yes_no, putln};
// use mynibun::nibun;

fn usize_to_chars(x: usize, n: usize) -> Vec<u8> {
    let mut v = vec![];
    let mut x = x;

    for _ in 0..n {
        v.push((x % 10) as u8);
        x /= 10;
    }
    v.sort();
    v
}

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let lg: usize = s.iter().sorted().rev().join("").parse().unwrap();
    let lg_sqrt = (lg as f64).sqrt().ceil() as usize;

    let s = usize_to_chars(lg ,n);

    let mut ans = 0;

    for i in 0..=lg_sqrt {
        let x = i * i;
        let v = usize_to_chars(x, n);

        if s == v {
            ans += 1;
        }
    }

    println!("{}", ans);
}
