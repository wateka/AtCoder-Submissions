use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
    }

    if n % 2 == 1 {
        return;
    }

    for p in (0..n).combinations(n/2) {
        let cand = (0..n).map(|i| if p.contains(&i) { '(' } else { ')' }).collect_vec();
        let mut right = true;
        let mut st = vec![];
        for c in cand.iter() {
            if *c == '(' {
                st.push(c);
            } else if st.pop().is_none() {
                right = false;
                break;
            }
        }

        if right {
            println!("{}", cand.into_iter().collect::<String>());
        }
    }
}
