use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(usize, usize); m],
    }

    let mut graph = vec![vec![]; n];
    for (u, v) in edges {
        graph[u - 1].push(v - 1);
        graph[v - 1].push(u - 1);
    }

    let mut visited = vec![false; n];
    let mut count = 0;

    for i in 0..n {
        if !visited[i] {
            count += 1;
            dfs(&graph, &mut visited, i);
        }
    }

    println!("{}", count)
}

fn dfs(graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>, node: usize) {
    let mut stack = VecDeque::new();
    stack.push_back(node);

    while let Some(current) = stack.pop_back() {
        if visited[current] {
            continue;
        }
        visited[current] = true;
        for &neighbor in &graph[current] {
            if !visited[neighbor] {
                stack.push_back(neighbor);
            }
        }
    }
}
