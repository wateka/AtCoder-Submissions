#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]

use std::io::{stdin, stdout, BufReader, Write};
use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
use ac_library::{dsu::Dsu, modint::ModInt998244353 as Mint9, modint::Mod1000000007 as Mint10};
use itertools::{Itertools, izip, iproduct};
use superslice::Ext;
use proconio::{fastout, input, marker::{Chars, Usize1}, source::line::LineSource};
macro_rules! yes_no { ($condition: expr) => {println!("{}", if $condition {"Yes"} else {"No"})} }
macro_rules! putln { ($value: expr) => {println!("{}", $value)} }

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));


    input! {
        from &mut source,
        n: usize,
    }

    let mut m = 1;
    while n > 1<<m {
        m += 1;
    }

    println!("{}", m);
    for i in 0..m {
        let mut drink = vec![];
        for j in 0..n {
            if (j / (1<<i)) % 2 == 1 {
                drink.push(j+1);
            }
        }
        println!("{} {}", drink.len(), drink.into_iter().join(" "));
    }
    stdout().flush().unwrap();

    input! {
        from &mut source,
        s: Chars,
    }

    let mut ans = 0;
    for c in s.into_iter().rev() {
        ans = ans<<1;
        if c == '1' {
            ans += 1;
        }
    }

    println!("{}", ans+1);
    stdout().flush().unwrap();
}
