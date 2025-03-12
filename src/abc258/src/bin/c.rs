use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        queries: [(usize, usize); q],
    }

    let mut offset = 0;

    for (t, x) in queries {
        if t == 1 {
            offset = (offset + n - x % n) % n;
        } else {
            println!("{}", s[(offset + x - 1) % n]);
        }
    }
}
