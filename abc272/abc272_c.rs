#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }

    a.sort();

    let a_even = a.clone().into_iter().filter(|ai| ai % 2 == 0).collect_vec();
    let a_odd = a.clone().into_iter().filter(|ai| ai % 2 == 1).collect_vec();

    let mut max = -1;

    if a_even.len() >= 2 {
        let e_max1 = a_even[a_even.len()-1];
        let e_max2 = a_even[a_even.len()-2];
        max = max.max(e_max1 + e_max2);
    }

    if a_odd.len() >= 2 {
        let o_max1 = a_odd[a_odd.len()-1];
        let o_max2 = a_odd[a_odd.len()-2];
        max = max.max(o_max1 + o_max2);
    }

    println!("{}", max);
}
