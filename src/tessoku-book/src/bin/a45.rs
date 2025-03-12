use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        c: String,
        a: Chars,
    }

    let mut score = 0;

    for i in 0..n {
        if a[i] == 'W' {
            score += 0;
        } else if a[i] == 'B' {
            score += 1;
        } else if a[i] == 'R' {
            score += 2;
        }
    }

    if score % 3 == 0 && c == "W" {
        println!("Yes")
    } else if score % 3 == 1 && c == "B" {
        println!("Yes")
    } else if score % 3 == 2 && c == "R" {
        println!("Yes")
    } else {
        println!("No")
    }
}
