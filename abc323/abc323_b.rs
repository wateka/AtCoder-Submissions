#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
use proconio::{input, marker::Chars};
// use myoutput::{print_yes_no, putln};
// use mynibun::nibun;

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let mut win = vec![0; n];
    for (player, si) in s.into_iter().enumerate() {
        for sij in si {
            if sij == 'o' {
                win[player]  += 1;
            }
        }
    }

    let win = win;
    let mut win_counts = vec![Vec::new(); n];

    for (player_index, count) in win.into_iter().enumerate() {
        win_counts[count].push(player_index+1);
    }

    let mut f = false;
    for ps in win_counts.into_iter().rev() {
        for p in ps {
            if f { print!(" "); } else { f = true; }
            print!("{}", p);
        }
    }
    println!();
}
