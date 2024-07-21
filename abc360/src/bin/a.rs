use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    if s[0] == 'R' || (s[1] == 'R' && s[2] == 'M') {
        println!("Yes");
    } else {
        println!("No");
    }
}
