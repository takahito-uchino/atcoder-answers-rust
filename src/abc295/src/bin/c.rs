use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut map = HashMap::new();

    for i in a {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    let mut answer = 0;

    for (_, v) in map {
        answer += v / 2;
    }

    println!("{}", answer)
}
