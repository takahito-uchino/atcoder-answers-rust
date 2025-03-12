use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let n = s.len();

    for i in 0..n {
        if s[i] != t[i] {
            println!("{}", i + 1);
            return;
        }
        if i == n - 1 && s[i] == t[i] {
            println!("{}", i + 2);
            return;
        }
    }
}
