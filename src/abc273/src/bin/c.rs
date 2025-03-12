use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut map = HashMap::new();

    for i in a {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    let mut vec: Vec<_> = map.into_iter().collect();

    vec.sort_by(|a, b| b.0.cmp(&a.0));

    for (_, v) in &vec {
        println!("{}", v);
    }

    for _ in 0..n - vec.len() {
        println!("0");
    }
}
