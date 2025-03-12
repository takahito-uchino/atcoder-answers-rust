use std::usize;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        s: Chars,
    }

    let mut doubled_s = s.clone();
    doubled_s.extend(&s);

    let mut ans = usize::MAX;

    for i in 0..n {
        let mut tmp = a * i;
        for j in 0..n / 2 {
            if doubled_s[i + j] != doubled_s[i + n - 1 - j] {
                tmp += b;
            }
        }
        ans = ans.min(tmp);
    }

    println!("{}", ans)
}
