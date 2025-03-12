use std::{collections::HashSet, iter::FromIterator};

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let a_set: HashSet<usize> = HashSet::from_iter(a.iter().cloned());

    for i in 0..k {
        if !a_set.contains(&i) {
            println!("{}", i);
            return;
        }
    }

    println!("{}", k)
}
