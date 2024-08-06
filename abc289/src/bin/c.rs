use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut a = Vec::new();

    for _ in 0..m {
        input! {
            _c: usize,
            ai: [usize; _c],
        }

        a.push(ai);
    }

    let mut answer = 0;

    for b in 0..(1 << m) {
        let mut s = HashSet::new();
        for i in 0..m {
            if (b >> i) & 1 == 1 {
                for &x in &a[i] {
                    s.insert(x);
                }
            }
        }
        if s.len() == n {
            answer += 1;
        }
    }

    println!("{}", answer)
}
