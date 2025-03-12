use std::usize;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut h: [usize; n],
    }

    h.sort_unstable();

    let mut answer = usize::MAX;

    for i in 0..(n - k + 1) {
        answer = answer.min(h[i + k - 1] - h[i]);
    }

    println!("{}", answer)
}
