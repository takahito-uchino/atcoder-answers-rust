use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        xk: [(usize, usize); q],
    }

    let mut map = HashMap::new();

    for (index, element) in a.iter().enumerate() {
        map.entry(element).or_insert_with(Vec::new).push(index + 1);
    }

    for (x, k) in xk {
        match map.get(&x) {
            Some(indices) if k <= indices.len() => {
                println!("{}", indices[k - 1]);
            }
            _ => {
                println!("-1")
            }
        }
    }
}
