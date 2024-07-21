use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: String,
    }

    for w in 1..s.len() {
        for c in 0..w {
            let mut str_now = "".to_string();

            let mut i = c;

            while i < s.len() {
                str_now.push(s[i]);
                i += w;
            }

            if str_now == t {
                println!("Yes");
                return;
            }
        }
    }

    println!("No")
}
