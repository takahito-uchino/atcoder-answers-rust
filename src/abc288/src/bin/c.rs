use std::collections::{HashSet, VecDeque};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(usize, usize); m],
    }

    let mut graph = vec![Vec::<usize>::new(); n];

    for (a, b) in edges {
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }

    let mut visited = HashSet::new();
    let mut s = 0;

    for node in 0..n {
        if !visited.contains(&node) {
            s += 1;
            let mut queue = VecDeque::new();
            queue.push_back(node);

            while let Some(current) = queue.pop_front() {
                for &neighbor in &graph[current] {
                    if !visited.contains(&neighbor) {
                        visited.insert(neighbor);
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    println!("{}", m + s - n)
}
