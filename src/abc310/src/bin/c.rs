use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let mut set = HashSet::new();

    for c in s {
        let mut c_rev = c.clone();
        c_rev.reverse();
        if !set.contains(&c) && !set.contains(&c_rev) {
            set.insert(c);
        }
    }

    println!("{}", set.len())
}
