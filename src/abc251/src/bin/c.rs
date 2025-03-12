use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        st: [(String, usize); n],
    }

    let mut appeared = HashSet::new();
    let mut best = 0;
    let mut best_score = 0;

    for (i, (s, t)) in st.iter().enumerate() {
        if appeared.contains(s) {
            continue;
        }
        appeared.insert(s);
        if best_score < *t {
            best = i;
            best_score = *t;
        }
    }

    println!("{}", best + 1)
}
