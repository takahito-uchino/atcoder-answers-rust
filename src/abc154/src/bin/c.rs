use proconio::input;
use std::{collections::HashSet, iter::FromIterator};

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let set: HashSet<usize> = HashSet::from_iter(a.iter().cloned());
    println!("{}", if n == set.len() { "YES" } else { "NO" })
}
