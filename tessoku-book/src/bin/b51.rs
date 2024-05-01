use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut answer = Vec::new();

    for i in 0..s.len() {
        if s[i] == '(' {
            answer.push(i);
        } else {
            println!("{} {}", answer.pop().unwrap() + 1, i + 1);
        }
    }
}
