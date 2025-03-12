use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut s = BTreeMap::new();

    for _ in 0..q {
        input! {
            query: usize,
        }

        if query == 1 {
            input! {
                x: usize,
            }
            let count = s.entry(x).or_insert(0);
            *count += 1;
        } else if query == 2 {
            input! {
                x: usize,
                c: usize,
            }
            if let Some(count) = s.get_mut(&x) {
                *count -= c.min(*count);
                if *count == 0 {
                    s.remove(&x);
                }
            }
        } else {
            if s.is_empty() {
                println!("EMPTY");
            } else {
                let min = s.keys().next().unwrap();
                let max = s.keys().next_back().unwrap();
                println!("{}", max - min)
            }
        }
    }
}
