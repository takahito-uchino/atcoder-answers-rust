use std::{collections::BTreeSet, iter::FromIterator};

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let s_set = BTreeSet::from_iter(s);

    for str in &s_set {
        let ss = format!("!{}", str);
        if s_set.contains(&ss) {
            println!("{}", str);
            return;
        }
    }

    println!("satisfiable")
}
