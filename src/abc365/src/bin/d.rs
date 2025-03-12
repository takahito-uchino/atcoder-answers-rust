use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut t = vec!['R'; n];
    if s[0] == 'R' {
        t[0] = 'P';
    } else if s[0] == 'P' {
        t[0] = 'S';
    } else if s[0] == 'S' {
        t[0] = 'R';
    }

    for i in 1..n {
        if s[i] == 'R' {
            if t[i - 1] == 'P' {
                t[i] = 'R';
            } else {
                t[i] = 'P';
            }
        } else if s[i] == 'P' {
            if t[i - 1] == 'S' {
                t[i] = 'P';
            } else {
                t[i] = 'S';
            }
        } else if s[i] == 'S' {
            if t[i - 1] == 'R' {
                t[i] = 'S';
            } else {
                t[i] = 'R';
            }
        }
    }

    let mut answer = 0;

    for i in 0..n {
        if s[i] != t[i] {
            answer += 1;
        }
    }

    println!("{}", answer)
}
