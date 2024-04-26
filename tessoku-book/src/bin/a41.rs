use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut answer = false;

    for i in 0..n-2 {
        if s[i]=='R' && s[i+1] == 'R' && s[i+2] == 'R' {
            answer = true;
        }
        if s[i]=='B' && s[i+1] == 'B' && s[i+2] == 'B' {
            answer = true;
        }
    }

    println!("{}", if answer {"Yes"} else {"No"})
}
