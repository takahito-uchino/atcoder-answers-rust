use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    }

    let mut answer = 0;

    for mask in 0..1 << n.len() {
        let mut s = Vec::new();
        let mut t = Vec::new();

        for i in 0..n.len() {
            if (mask >> i & 1) == 1 {
                s.push(n[i]);
            } else {
                t.push(n[i]);
            }
        }

        s.sort();
        t.sort();

        let s = s
            .iter()
            .rev()
            .collect::<String>()
            .parse::<usize>()
            .unwrap_or(0);
        let t = t
            .iter()
            .rev()
            .collect::<String>()
            .parse::<usize>()
            .unwrap_or(0);
        answer = answer.max(s * t);
    }

    println!("{}", answer)
}
