use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut table = BTreeSet::new();

    for _ in 0..q {
        input! {
            n: usize,
            x: usize,
        }

        if n == 1 {
            table.insert(x);
        } else if n == 2 {
            let mut result = None;
            if let Some(&min) = table.range(x..).next() {
                result = Some(min - x);
            }
            if let Some(&max) = table.range(..=x).next_back() {
                result = match result {
                    Some(existing) => Some(existing.min(x - max)),
                    None => Some(x - max),
                };
            }
            match result {
                Some(min_diff) => println!("{}", min_diff),
                None => println!("-1"),
            }
        }
    }
}
