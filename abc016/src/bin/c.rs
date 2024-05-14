use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut g = vec![Vec::new(); n];

    for (a, b) in ab {
        g[a - 1].push(b - 1);
        g[b - 1].push(a - 1);
    }

    for v in 0..n {
        let mut s = HashSet::new();

        for &i in &g[v] {
            for j in &g[i] {
                s.insert(j);
            }
        }

        s.remove(&v);
        for i in &g[v] {
            s.remove(&i);
        }

        println!("{}", s.len())
    }
}
