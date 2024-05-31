use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut map = BTreeMap::new();

    for si in s {
        let count = map.entry(si).or_insert(0);
        *count += 1;
    }

    let max = map.values().max().unwrap();

    for (str, v) in &map {
        if v == max {
            println!("{}", str)
        }
    }
}
