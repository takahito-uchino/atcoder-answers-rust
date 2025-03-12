use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        d: f64,
        xy: [(f64, f64); n],
    }

    let mut graph = vec![vec![false; n]; n];

    for i in 0..n {
        for j in 0..n {
            if get_distance(xy[i].0, xy[j].0, xy[i].1, xy[j].1) <= d {
                graph[i][j] = true;
            }
        }
    }
    let mut answer = vec![false; n];
    answer[0] = true;
    let mut queue = VecDeque::new();
    queue.push_back(0);
    while let Some(q) = queue.pop_front() {
        for i in 0..n {
            if graph[q][i] && !answer[i] {
                answer[i] = true;
                queue.push_back(i);
            }
        }
    }
    for i in 0..n {
        if answer[i] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

fn get_distance(a1: f64, b1: f64, a2: f64, b2: f64) -> f64 {
    ((a1 - b1) * (a1 - b1) + (a2 - b2) * (a2 - b2)).sqrt()
}
