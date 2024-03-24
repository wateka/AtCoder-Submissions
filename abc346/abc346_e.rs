use proconio::{fastout, input, marker::Usize1};
use itertools::Itertools;
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        m: usize,
        queries: [(usize, Usize1, usize); m],
    }

    let mut row = vec![0; h];
    let mut col = vec![0; w];
    let mut color_of = vec![0; m+1];

    for (i, (ti, ai, xi)) in queries.into_iter().enumerate() {
        color_of[i+1] = xi;
        match ti {
            1 => row[ai] = i+1,
            2 => col[ai] = i+1,
            _ => unreachable!(),
        }
    }

    row.sort();
    col.sort();

    let mut counts = vec![0; 200001];

    for &ri in row.iter() {
        counts[color_of[ri]] += col.upper_bound(&ri);
    }

    for &ci in col.iter() {
        counts[color_of[ci]] += row.upper_bound(&ci);
    }

    counts[0] = h * w - counts[1..].iter().sum::<usize>();

    let res = counts.into_iter().enumerate().filter(|&(_, count)| count > 0);
    println!("{}", res.clone().count());
    println!("{}", res.map(|(i, count)| format!("{} {}", i, count)).join("\n"));
}