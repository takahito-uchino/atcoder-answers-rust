use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut queries = Vec::new();

    for _ in 0..q {
        input! {
            n: usize,
        }
        if n == 1 || n == 2 {
            input! {
                x: usize,
            }
            queries.push((n, x));
        } else if n == 3 {
            queries.push((n, 0));
        }
    }

    let mut map = HashMap::new();

    for (i, x) in queries {
        if i == 1 {
            let count = map.entry(x).or_insert(0);
            *count += 1;
        } else if i == 2 {
            let count = map.entry(x).or_insert(0);
            *count -= 1;
            if *count == 0 {
                map.remove(&x);
            }
        } else if i == 3 {
            println!("{}", map.len());
        }
    }
}
