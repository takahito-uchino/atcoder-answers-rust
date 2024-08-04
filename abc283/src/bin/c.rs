use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut answer = s.len();
    let mut i = 0;

    while i < s.len() - 1 {
        if s[i] == '0' && s[i + 1] == '0' {
            answer -= 1;
            i += 2;
        } else {
            i += 1;
        }
    }

    println!("{}", answer)
}
