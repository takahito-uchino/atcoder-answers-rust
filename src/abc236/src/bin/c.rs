use std::{collections::HashSet, iter::FromIterator};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; n],
        t: [String; m],
    }

    let t_set: HashSet<String> = HashSet::from_iter(t);

    for station in s {
        println!(
            "{}",
            if t_set.contains(&station) {
                "Yes"
            } else {
                "No"
            }
        )
    }
}
