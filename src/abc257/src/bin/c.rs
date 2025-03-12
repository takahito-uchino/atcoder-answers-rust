use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
        w: [usize; n],
    }

    let mut a: Vec<(usize, char)> = Vec::new();
    let mut ans = 0;

    for (i, &x) in w.iter().enumerate() {
        a.push((x, s[i]));
        if s[i] == '1' {
            ans += 1;
        }
    }

    a.sort_by(|a, b| a.0.cmp(&b.0));
    let mut x = ans;

    for i in 0..n {
        if a[i].1 == '1' {
            x -= 1;
        } else {
            x += 1;
        }

        if i < n - 1 {
            if a[i].0 != a[i + 1].0 {
                ans = ans.max(x);
            }
        } else {
            ans = ans.max(x);
        }
    }

    println!("{}", ans);
}
