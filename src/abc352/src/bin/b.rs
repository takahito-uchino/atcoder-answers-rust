use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut current = 0;
    let mut answer = Vec::new();

    for i in 0..t.len() {
        if s[current] == t[i] {
            answer.push(i + 1);
            current += 1;
        }
    }

    println!("{}", answer.iter().map(|&num| num.to_string()).join(" "))
}
