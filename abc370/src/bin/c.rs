use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
        t: Chars,
    }

    let mut x = Vec::new();

    for i in 0..t.len() {
        if s[i] > t[i] {
            s[i] = t[i];
            let str: String = s.iter().collect();
            x.push(str);
        }
    }

    for i in (0..t.len()).rev() {
        if s[i] < t[i] {
            s[i] = t[i];
            let str: String = s.iter().collect();
            x.push(str);
        }
    }

    println!("{}", x.len());
    for s in x {
        println!("{}", s);
    }
}
