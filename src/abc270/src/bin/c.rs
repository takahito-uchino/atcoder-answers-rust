use proconio::input;
use std::collections::VecDeque;

fn dfs(
    k: usize,
    to: usize,
    e: &Vec<Vec<usize>>,
    flag: &mut Vec<bool>,
    deq: &mut VecDeque<usize>,
    stop: &mut bool,
) {
    if !*stop {
        deq.push_back(k);
    }
    if k == to {
        *stop = true;
    }
    flag[k] = true;
    for &next in &e[k] {
        if !flag[next] {
            dfs(next, to, e, flag, deq, stop);
        }
    }
    if !*stop {
        deq.pop_back();
    }
}

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        edges: [(usize, usize); n - 1],
    }

    let mut e = vec![vec![]; n + 1];

    for (u, v) in edges {
        e[u].push(v);
        e[v].push(u);
    }

    let mut flag = vec![false; n + 1];
    let mut deq: VecDeque<usize> = VecDeque::new();
    let mut stop = false;

    dfs(x, y, &e, &mut flag, &mut deq, &mut stop);

    while let Some(front) = deq.pop_front() {
        print!("{}", front);
        if deq.is_empty() {
            println!();
        } else {
            print!(" ");
        }
    }
}
