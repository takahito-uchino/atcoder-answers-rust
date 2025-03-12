use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut map = HashMap::new();

    for _ in 0..q {
        input! {
            n: usize,
        }

        if n == 1 {
            input! {
                name: String,
                score: usize,
            }

            map.insert(name, score);
        } else {
            input! {
                name: String,
            }

            println!("{}", map.get(&name).unwrap())
        }
    }
}
