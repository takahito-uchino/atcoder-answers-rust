use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let l = s.len();
    let mut result = 0;
    let mut add = 26;
    for _ in 1..l {
        result += add;
        add *= 26;
    }
    let mut num = 0;
    for i in 0..l {
        num *= 26;
        num += s[i] as usize - 'A' as usize;
    }

    println!("{}", result + num + 1);
}
