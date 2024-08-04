use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let mut wrap = false;

    for c in s {
        if c == '"' && !wrap {
            wrap = true;
            print!("\"");
        } else if c == '"' && wrap {
            wrap = false;
            print!("\"");
        } else if c == ',' && !wrap {
            print!(".");
        } else {
            print!("{}", c);
        }
    }

    println!()
}
