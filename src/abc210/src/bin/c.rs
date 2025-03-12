use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        c: [usize; n],
    }

    let mut map = HashMap::new();

    for i in 0..k {
        *map.entry(c[i]).or_insert(0) += 1;
    }

    let mut answer = map.len();

    for i in k..n {
        *map.entry(c[i]).or_insert(0) += 1;
        let count = map.entry(c[i - k]).or_insert(0);
        *count -= 1;
        if *count == 0 {
            map.remove(&c[i - k]);
        }

        answer = answer.max(map.len());
    }

    println!("{}", answer)
}
