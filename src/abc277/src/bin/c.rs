use std::collections::{HashMap, HashSet, VecDeque};

use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }

    let mut graph = HashMap::new();

    for (a, b) in ab {
        graph.entry(a).or_insert(Vec::new()).push(b);
        graph.entry(b).or_insert(Vec::new()).push(a);
    }

    let mut que = VecDeque::new();
    que.push_back(1);
    let mut visited = HashSet::new();
    visited.insert(1);

    while let Some(v) = que.pop_front() {
        if let Some(neighbors) = graph.get(&v) {
            for &i in neighbors {
                if !visited.contains(&i) {
                    que.push_back(i);
                    visited.insert(i);
                }
            }
        }
    }

    if let Some(max_value) = visited.iter().max() {
        println!("{}", max_value);
    }
}
