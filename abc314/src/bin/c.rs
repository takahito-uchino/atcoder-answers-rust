use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        c: [usize; n],
    }

    let mut p = vec![Vec::new(); m + 1];
    for i in 0..n {
        p[c[i]].push(i);
    }

    let mut t = vec!['?'; n];

    for i in 1..=m {
        let k = p[i].len();
        for j in 0..k {
            t[p[i][(j + 1) % k]] = s[p[i][j]];
        }
    }

    println!("{}", t.into_iter().collect::<String>())
}
