use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut answer = HashSet::new();

    for mut ai in a {
        while ai % 2 == 0 {
            ai /= 2;
        }
        answer.insert(ai);
    }

    println!("{}", answer.len())
}
