use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let answer = rlc(&s);

    println!("{}", answer.len())
}

fn rlc(s: &[char]) -> Vec<char> {
    let mut i = 0;
    let mut ctr = vec![];
    let mut cur = s[0];

    loop {
        while i < s.len() && s[i] == cur {
            i += 1;
        }
        ctr.push(cur);
        if i == s.len() {
            break;
        } else {
            cur = s[i];
        }
    }

    ctr
}
