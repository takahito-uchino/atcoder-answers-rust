use std::collections::{HashMap, VecDeque};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize, usize); m],
    }

    let mut graph = HashMap::new();

    for (u, v) in uv {
        graph.entry(u - 1).or_insert(Vec::new()).push(v - 1);
        graph.entry(v - 1).or_insert(Vec::new()).push(u - 1);
    }

    if m != n - 1 {
        println!("No");
        return;
    }

    for i in 0..n {
        if let Some(vec) = graph.get(&i) {
            if vec.len() > 2 {
                println!("No");
                return;
            }
        }
    }

    let mut reach = vec![false; n];
    let mut que = VecDeque::new();
    reach[0] = true;
    que.push_back(0);

    while let Some(u) = que.pop_front() {
        if let Some(neighbors) = graph.get(&u) {
            for &v in neighbors {
                if !reach[v] {
                    reach[v] = true;
                    que.push_back(v);
                }
            }
        }
    }

    for v in reach {
        if !v {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
