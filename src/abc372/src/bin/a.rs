use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    for i in 0..s.len() {
        if s[i] == '.' {
            continue;
        }
        print!("{}", s[i]);
    }

    println!()
}
