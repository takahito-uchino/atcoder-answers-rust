use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut table = HashSet::new();

    for _ in 0..q {
        input! {
            n: usize,
            num: usize,
        }

        if n == 1 {
            table.insert(num);
        } else if n == 2 {
            table.remove(&num);
        } else if n == 3 {
            let itr = table.iter().filter(|&&m| m >= num).min();
            match itr {
                Some(&min) => println!("{}", min),
                None => println!("-1"),
            }
        }
    }
}
