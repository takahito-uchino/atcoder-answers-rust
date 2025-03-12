use std::usize;

use proconio::input;

fn main() {
    input! {
        n: usize,
        h: usize,
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
    }

    let mut answer = usize::MAX;

    for x in 0..=n {
        for y in 0..=(n - x) {
            if h + b * x + d * y > (n - x - y) * e {
                answer = answer.min(a * x + c * y);
            }
        }
    }

    println!("{}", answer)
}
