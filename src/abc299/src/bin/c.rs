use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: Chars,
    }

    let mut answer = 0;

    for _ in 0..2 {
        let mut level = 0;
        for i in 0..n {
            if s[i] == '-' {
                answer = answer.max(level);
                level = 0;
            } else {
                level += 1;
            }
        }
        s.reverse();
    }

    if answer == 0 {
        println!("-1");
    } else {
        println!("{}", answer)
    }
}
