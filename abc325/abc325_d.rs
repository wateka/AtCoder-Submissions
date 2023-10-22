#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::{collections::{HashMap, HashSet, BTreeMap, BTreeSet, BinaryHeap}, cmp::Reverse};
use proconio::{input, marker::Chars};
// use myoutput::{print_yes_no, putln};
// use mynibun::nibun;

fn main() {
    input! {
        n: usize,
        td: [(i64, i64); n],
    }

    let mut waitings: BinaryHeap<Reverse<(i64, i64)>> = td.into_iter().map(|tdi| Reverse(tdi)).collect();
    let mut printables: BinaryHeap<Reverse<i64>> = BinaryHeap::new();
    let mut printed_count = 0;

    let mut cur = 0;
    loop {
        // 印字開始時刻の商品を、印字機に入れる
        loop {
            let Some(&Reverse((ti, di))) = waitings.peek() else { break };
            if ti == cur {
                waitings.pop();
                printables.push(Reverse(ti + di));
            } else {
                break;
            }
        }

        // 印字できなかった商品を取り出す
        loop {
            let Some(&Reverse(limit)) = printables.peek() else { break };
            if limit < cur {
                printables.pop();
            } else {
                break;
            }
        }

        // 商品に印字する
        if printables.pop().is_some() {
            printed_count += 1;
        }

        // 時間を更新する
        cur += 1;
        if printables.is_empty() {
            if waitings.is_empty() {
                break;
            } else {
                Reverse((cur, _)) = *waitings.peek().unwrap();
            }
        }
    }

    println!("{}", printed_count);
}
