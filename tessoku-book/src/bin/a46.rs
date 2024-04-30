use std::usize;

use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(f64, f64); n],
    }

    let answer = play_greedy(n, xy);

    for i in answer {
        println!("{}", i + 1)
    }
}

fn get_distance(i: (f64, f64), j: (f64, f64)) -> f64 {
    ((i.0 - j.0).powi(2) + (i.1 - j.1).powi(2)).sqrt()
}

fn play_greedy(n: usize, xy: Vec<(f64, f64)>) -> Vec<usize> {
    let mut current_place = 0;
    let mut visited = vec![false; n];
    visited[0] = true;
    let mut p = Vec::new();
    p.push(0);

    for _ in 1..n {
        let mut min_dist = f64::INFINITY;
        let mut min_id = usize::MAX;

        for j in 0..n {
            let distance = get_distance(xy[current_place], xy[j]);
            if !visited[j] && min_dist > distance {
                min_dist = distance;
                min_id = j;
            }
        }

        visited[min_id] = true;
        p.push(min_id);
        current_place = min_id;
    }

    p.push(0);
    p
}
