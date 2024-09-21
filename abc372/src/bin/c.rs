use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        q: usize,
        mut s: Chars,
        queries: [(usize, char); q],
    }

    let mut count = 0;

    for i in 0..s.len() - 2 {
        if s[i] == 'A' && s[i + 1] == 'B' && s[i + 2] == 'C' {
            count += 1;
        }
    }

    for (x, c) in queries {
        let idx = x - 1;

        let start = if idx >= 2 { idx - 2 } else { 0 };
        let end = (idx + 2).min(s.len() - 1);

        for i in start..=end {
            if i + 2 < s.len() && s[i] == 'A' && s[i + 1] == 'B' && s[i + 2] == 'C' {
                count -= 1;
            }
        }

        s[idx] = c;

        for i in start..=end {
            if i + 2 < s.len() && s[i] == 'A' && s[i + 1] == 'B' && s[i + 2] == 'C' {
                count += 1;
            }
        }

        println!("{}", count);
    }
}
