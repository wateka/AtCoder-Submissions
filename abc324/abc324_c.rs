#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
use proconio::{input, marker::Chars};
use rand::seq::index;
use itertools::Itertools;
// use myoutput::{print_yes_no, putln};
// use mynibun::nibun;

fn main() {
    input! {
        n: usize,
        mut td: Chars,
        s: [Chars; n],
    }

    

    let mut indexes: Vec<usize> = vec![];

    for (i, si) in s.into_iter().enumerate() {
        let mut si = si;
        let mut d = 0;
        if si.len() == td.len() {
            for (sij, tij) in td.iter().zip(si) {
                if *sij != tij {
                    d += 1;
                    if d > 1{
                        break;
                    }
                }
            }
        } else if si.len()+1 == td.len() {
            for j in 0..td.len() {
                if j-d >= si.len() {
                    d += 1;
                    break;
                }
                if td[j] != si[j-d] {
                    d += 1;
                    if d > 1{
                        break;
                    }
                }
            }
        } else if si.len() == td.len()+1 {
            for j in 0..si.len() {
                if j-d >= td.len() {
                    d += 1;
                    break;
                }
                if si[j] != td[j-d] {
                    d += 1;
                    if d > 1{
                        break;
                    }
                }
            }
        } else {
            continue;
        }

        if d < 2 {
            indexes.push(i+1);
        }
    }

    println!("{}", indexes.len());
    println!("{}", indexes.iter().join(" "));
}
