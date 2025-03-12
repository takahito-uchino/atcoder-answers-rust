use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }

    let mut c: Vec<usize> = Vec::new();
    c.extend(a.iter());
    c.extend(b.iter());
    c.sort_unstable();

    let mut get_index = HashMap::new();

    for (i, v) in c.iter().enumerate() {
        get_index.insert(v, i + 1);
    }

    for i in a {
        print!("{} ", get_index[&i]);
    }

    println!();

    for i in b {
        print!("{} ", get_index[&i])
    }
}
