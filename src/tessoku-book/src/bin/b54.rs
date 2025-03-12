use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut map = HashMap::new();
    let mut answer = 0usize;

    for &v in &a {
        let count = map.entry(v).or_insert(0);
        answer += *count;
        *count += 1;
    }

    println!("{}", answer)
}
