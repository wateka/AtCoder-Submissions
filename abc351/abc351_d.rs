#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]

use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
use ac_library::{dsu::Dsu, modint::ModInt998244353 as Mint9, modint::Mod1000000007 as Mint10};
use itertools::{Itertools, izip, iproduct};
use superslice::Ext;
use proconio::{fastout, input, marker::{Chars, Usize1}};
macro_rules! yes_no { ($condition: expr) => {println!("{}", if $condition {"Yes"} else {"No"})} }
macro_rules! putln { ($value: expr) => {println!("{}", $value)} }

fn dfs(i: usize, j: usize, term: &Vec<Vec<bool>>, seen: &mut Vec<Vec<bool>>, temp_seen: &mut HashSet<(usize, usize)>, h: &usize, w: &usize) -> usize {
    if seen[i][j] || temp_seen.contains(&(i, j)) {
        return 0;
    } else if term[i][j] {
        temp_seen.insert((i, j));
        return 1;
    } else {
        // eprintln!("{} {}", i, j);
        // eprintln!();

        seen[i][j] = true;

        let mut res = 1;

        if i < h-1 {
            res += dfs(i+1, j, term, seen, temp_seen, h, w);
        }
        if j < w-1 {
            res += dfs(i, j+1, term, seen, temp_seen, h, w);
        }
        if i > 0 {
            res += dfs(i-1, j, term, seen, temp_seen, h, w);
        }
        if j > 0 {
            res += dfs(i, j-1, term, seen, temp_seen, h, w);
        }

        return res;
    }
}

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut term = vec![vec![false; w]; h];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#'
            || (i < h-1 && s[i+1][j] == '#')
            || (j < w-1 && s[i][j+1] == '#')
            || (i > 0 && s[i-1][j] == '#')
            || (j > 0 && s[i][j-1] == '#') {
                term[i][j] = true;
            }
        }
    }

    let mut seen = vec![vec![false; w]; h];
    let mut ans = 1;
    for i in 0..h {
        for j in 0..w {
            if !term[i][j] && !seen[i][j] {
                // eprintln!("{} {}", i, j);
                let mut temp_seen = HashSet::new();
                let res = dfs(i, j, &term, &mut seen, &mut temp_seen, &h, &w);
                ans = ans.max(res);
            }
        }
    }

    putln!(ans);

    // for i in 0..h {
    //     for j in 0..w {
    //         eprint!("{}", if mag[i][j] { "x" } else if term[i][j] { "o" } else { "." });
    //     }
    //     eprintln!();
    // }
}
