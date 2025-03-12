use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use proconio::input;

fn main() {
    input! {
        n: usize,
        mg: usize,
        uv: [(usize, usize); mg],
        mh: usize,
        ab: [(usize, usize); mh],
    }

    let mut a = Vec::new();

    for i in 0..n - 1 {
        let len = n - i - 1;
        input! {
            _a: [usize; len],
        }
        a.push(_a);
    }

    let unique_edges = find_unique_edges(&uv, &ab);

    for (u, v) in unique_edges {
        println!("{} {}", u, v)
    }
}

fn find_unique_edges(
    edges1: &Vec<(usize, usize)>,
    edges2: &Vec<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let set1: HashSet<(usize, usize)> = edges1.iter().map(|&(u, v)| (u.min(v), u.max(v))).collect();
    let set2: HashSet<(usize, usize)> = edges2.iter().map(|&(u, v)| (u.min(v), u.max(v))).collect();

    set1.symmetric_difference(&set2).cloned().collect()
}
