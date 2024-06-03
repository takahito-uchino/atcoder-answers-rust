use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut answer = BTreeSet::new();

    let mut d = 1;

    while d * d <= n {
        if n % d == 0 {
            answer.insert(d);
            answer.insert(n / d);
        }
        d += 1;
    }

    for i in answer {
        println!("{}", i)
    }
}
