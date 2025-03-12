use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut set = HashSet::new();

    for str in s {
        set.insert(str);
    }

    println!("{}", set.len())
}
